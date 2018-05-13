//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! This module is the main routine of the ADS-B Simulator.  The main routine
//! is actually `producer()`.  It seems impossible to divide this into smaller
//! functions even for documentation purposes.
//!
//! In terms of message passing, the callback routine called for each MQTT
//! message received simply sends the message via a Rust channel to the
//! producer() thread.  The producer does all things message to avoid the need
//! for mutual exclusion mechanisms.
//!
use std::env;
use std::{thread, time};
use std::net::UdpSocket;
use std::sync::mpsc;

mod coords;
use coords::CwithV;

mod mqtt;
use mqtt::Client;

mod mavlink;
use mavlink::Message;

// ---------------------------------------------------------------------------

/// The main() routine parses CLI parameters and establishes commnunications
fn main () -> () {
    let mut mqtt = Client::new();

    for argument in env::args() {
        if argument.starts_with("-mq=") {
            mqtt.set_cli(&argument[4..]);
        }
    }

    let (sender, receiver) = mpsc::channel();

//    mqtt.dump();
    mqtt.connect();

    let mut mqttpub = mqtt.clone();

    thread::spawn(move || {
        producer(&receiver, &mut mqttpub);
    });

    println!("Consumer {}", mqtt.subtopic);
    mqtt.subscribe(&sender, |channel, mavmsg| {channel.send(Vec::from(mavmsg)).unwrap();});

    mqtt.disconnect();

    println!("Goodbye cruel, world!");
}

// ---------------------------------------------------------------------------

/// The producer() thread publishes all messages and handles messages received
fn producer(channel: &mpsc::Receiver<Vec<u8>>, mqtt: &mut mqtt::Client) -> () {
    // Two options, one routine
    let mut inet: Option<UdpSocket> = None;

    // Position and velcity of the UAV and its nemesis
    let mut uav = CwithV::new();
    let mut ufo = CwithV::new();

    // Can't beat a good old fashions Booleaed flag (otherwise I surrender with a white one)
    let mut ufoinitialised = false;

    // Only used by the INET option
    let mut dstaddress = String::new();

    // process remaing CLI parameters (mqtt parameters already done)
    for argument in env::args() {
        if argument.starts_with("-uav=") {
            uav.set_cli(&argument[5..]);
        }
        if argument.starts_with("-ufo=") {
            ufo.set_cli(&argument[5..]);
            ufoinitialised = true;
        }
        if argument.starts_with("-i=") {
            let fission: Vec<&str> = argument[3..].split(':').collect();

            let ouraddress = "127.0.0.1".to_owned() + ":" + fission[0];

            if fission.len() == 2 || fission[2].is_empty() {
                dstaddress = "127.0.0.1".to_owned() + ":" + fission[1];
            }
            else {
                dstaddress = fission[2].to_owned() + ":" + fission[1];
            }

            match UdpSocket::bind(ouraddress) {
                Ok(socket) => inet = Some(socket),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    // just to be clear where generated messages are going
    match inet {
        Some(_) => println!("Socket {}", dstaddress),
        None    => println!("Producer {}", mqtt.pubtopic),
    }

    // the MAVLink messages generated every second - avoid calling new each second
    let mut datastreamrequest   = mavlink::msg66::Message::new();
    let mut status              = mavlink::msg203::Message::new();
    let mut ownship             = mavlink::msg202::Message::new();
    let mut trafficreport       = mavlink::msg246::Message::new();

    // deduce the ICAO address to be used when generating 246 messages
    trafficreport.icao =
        match inet {
            Some(_) => 0x00300159,
            None    => u32::from_str_radix(mqtt.get_202_subtopic(), 16).unwrap(),
        };

    println!("ICAO: {:08x}", trafficreport.icao);

    // so the original course may be resumed when 84 messages stop coming in
    let uav_orig = uav.clone();

    // need to loop once a second
    let one_second = time::Duration::new(1, 0);

    loop {
        // record when this loop (iteration) starts
        let start = time::Instant::now();

        // update the ufo's position (possibly to be overridden by arrival of 202 message)
        ufo.update_position();

        // process any message that have arrived since last time
        for mut mavmsg in channel.try_iter() {
            match mavmsg[5] {
                84 => {
                    let mut settargetposition = mavlink::msg84::Message::new();

                    settargetposition.deserialise(&mavmsg);

                    uav.set_velocity(settargetposition.vx, settargetposition.vy, settargetposition.vz);
                }
                202 => {
                    let mut ownship = mavlink::msg202::Message::new();

                    ownship.deserialise(&mavmsg);

                    ownship.get_cwithv(&mut ufo);
                    ufoinitialised = true;
                }
                _ => {
                    println!("unexpected message {} ({})", mavmsg[5], mavmsg[2]);
                }
            }
        }

        // update the uav's position (once per loop means once per second)
        uav.update_position();

        // generate a burst of messages as would the real ADS-B device
        for msgid in [66, 203, 202, 246].iter() {
            let message = match *msgid {
                66 =>  {
                    datastreamrequest.serialise()
                },
                203 =>  {
                    status.serialise()
                },
                202 =>  {
                    ownship.set_cwithv(&uav).serialise()
                },
                246 =>  {
                    if ! ufoinitialised {continue;}

                    trafficreport.set_cwithv(&ufo).serialise()
                },
                _  =>  {
                    panic!("WTF: msgid = {}", msgid);
                },
            };

            match
                match inet {
                    Some(_) => inet.as_ref().unwrap().send_to(message, &dstaddress),
                    None    => mqtt.publish(message, &format!("/{}", msgid)),
                }
            {
                Ok(_) => (),
                Err(e) => panic!("Error: {}", e),
            };
        }

        // resume old course (unless next loop another 84 says otherwise)
        uav.set_course(&uav_orig);

        // sleep for the rest of the second and then start again
        thread::sleep(one_second - start.elapsed());
    }
}

// EOF
