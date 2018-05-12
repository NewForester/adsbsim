//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//!
//! This module is the main routine of the ADS-B Simulator divided into several
//! functions for documentation purposes.
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

extern crate mosquitto_client;

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

/*
    // no, it seems the mosquitto wrapper is not as thread safe as claimed

    let mut mqttsub = mqtt.clone();

    thread::spawn(move || {
        println!("Consumer {}", mqttsub.subtopic);
        consumer(&mut mqttsub);
    });
*/
    println!("Consumer {}", mqtt.subtopic);
    mqtt.subscribe(&sender, snake_case);

    mqtt.disconnect();

    println!("Goodbye cruel, world!");
}

fn snake_case(channel: &mpsc::Sender<Vec<u8>>, mavmsg: &[u8]) -> () {
    let v = Vec::from(mavmsg);

    channel.send(v).unwrap();
}

fn producer(channel: &mpsc::Receiver<Vec<u8>>, mqtt: &mut mqtt::Client) -> () {
    let mut inet: Option<UdpSocket> = None;

    let mut uav = CwithV::new();
    let mut ufo = CwithV::new();

    let mut ufoinitialised = false;

    let mut ouraddress = String::new();
    let mut dstaddress = String::new();

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

            ouraddress = "127.0.0.1".to_owned() + ":" + fission[0];

            if fission.len() == 2 || fission[2].is_empty() {
                dstaddress = "127.0.0.1".to_owned() + ":" + fission[1];
            }
            else {
                dstaddress = fission[2].to_owned() + ":" + fission[1];
            }

            match UdpSocket::bind(ouraddress.clone()) {
                Ok(socket) => inet = Some(socket),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    match inet {
        Some(_) => println!("Socket {}", ouraddress),
        None    => println!("Producer {}", mqtt.pubtopic),
    }

    let mut datastreamrequest   = mavlink::msg66::Message::new();
    let mut status              = mavlink::msg203::Message::new();
    let mut ownship             = mavlink::msg202::Message::new();
    let mut trafficreport       = mavlink::msg246::Message::new();

    trafficreport.icao =
        match inet {
            Some(_) => 0x00300100,
            None    => u32::from_str_radix(mqtt.get_202_subtopic(), 16).unwrap(),
        };

    println!("ICAO: {:08x}", trafficreport.icao);

    let one_second = time::Duration::new(1, 0);

    let uav_orig = uav.clone();

    loop {
        let start = time::Instant::now();

        ufo.update_position();

        for mut mavmsg in channel.try_iter() {
            match mavmsg[5] {
                84 => {
                    let mut settargetposition = mavlink::msg84::Message::new();

                    settargetposition.deserialise(&mavmsg);

                    let mut lre = CwithV::new();
                    lre.set_velocity(settargetposition.vx, settargetposition.vy, settargetposition.vz);
                    uav.set_course(&lre);
                }
                202 => {
                    let mut ownship = mavlink::msg202::Message::new();

                    ownship.deserialise(&mavmsg);

                    ownship.get_candv(&mut ufo);
                    ufoinitialised = true;
                }
                _ => {
                    println!("unexpected message {} ({})", mavmsg[5], mavmsg[2]);
                }
            }
        }

        uav.update_position();

        for msgid in [66, 203, 202, 246].iter() {
            let message = match *msgid {
                66 =>  {
                    datastreamrequest.serialise()
                },
                203 =>  {
                    status.serialise()
                },
                202 =>  {
                    ownship.set_candv(&uav).serialise()
                },
                246 =>  {
                    if ! ufoinitialised {continue;}

                    trafficreport.set_candv(&ufo).serialise()
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

        uav.set_course(&uav_orig);

        thread::sleep(one_second - start.elapsed());
    }
}

// EOF
