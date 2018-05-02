//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
use std::env;
use std::{thread, time};
use std::net::UdpSocket;
use std::io::{Error, ErrorKind};
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
        if mqttpub.pubtopic.len() != 0 {
            println!("Producer {}", mqttpub.pubtopic);
            producer(&receiver, &mut mqttpub);
        }
        else {
            println!("Socket");
            inet_producer(&receiver);
        }
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
    let mut uav = CwithV::new();
    let mut ufo = CwithV::new();

//    let mut ouraddress = String::new();
//    let mut dstaddress = String::new();

    for argument in env::args() {
        if argument.starts_with("-uav=") {
            uav.set_cli(&argument[5..]);
        }
        if argument.starts_with("-ufo=") {
            ufo.set_cli(&argument[5..]);
        }
//        if argument.starts_with("-i=") {
//            let fission: Vec<&str> = argument[3..].split(':').collect();

//            ouraddress = "127.0.0.1".to_owned() + ":" + fission[0];

//            if fission.len() == 2 || fission[2].is_empty() {
//                dstaddress = "127.0.0.1".to_owned() + ":" + fission[1];
//            }
//            else {
//                dstaddress = fission[2].to_owned() + ":" + fission[1];
//            }
//        }
    }

//    let socket: UdpSocket;
//    match UdpSocket::bind(ouraddress) {
//        Ok(nn) => socket = nn,
//        Err(e) => {println!("Error: {}", e); return;},
//    }

    let one_second = time::Duration::new(1, 0);

    let mut datastreamrequest   = mavlink::msg66::Message::new();
    let mut status              = mavlink::msg203::Message::new();
    let mut ownship             = mavlink::msg202::Message::new();
    let mut trafficreport       = mavlink::msg246::Message::new();

    trafficreport.icao = 0x00300100;

    let uav_orig = uav.clone();

    loop {
        let start = time::Instant::now();

        for msgid in [66, 203, 202, 246].iter() {
//            match
                match *msgid {
                    66 =>  {
                        mqtt.publish(datastreamrequest.serialise().message());
                    },
                    203 =>  {
                        mqtt.publish(status.serialise().message());
                    },
                    202 =>  {
                        mqtt.publish(ownship.set_candv(&uav).serialise().message());
                    },
                    246 =>  {
                        mqtt.publish(trafficreport.set_candv(&ufo).serialise().message());
                    },
                    _  =>  {
                        format!("WTF: msgid = {}", msgid);
                    },
                }
//            {
//                Ok(_) => (),
//                Err(e) => {println!("Error: {}", e); return;},
//            }
        }

        let mut lre = CwithV::new();
        let mut avoid = false;

        for mut mavmsg in channel.try_iter() {
            println!("received message {} ({})", mavmsg[5], mavmsg[2]);

            let mut settargetposition   = mavlink::msg84::Message::new();

            settargetposition.unpack_payload(&mavmsg).unwrap(); // tut tut

            lre.velocity(settargetposition.vx, settargetposition.vy, settargetposition.vz);
            avoid = true;
        }

        if avoid {
            uav.course(&lre);
        }
        else {
            uav.course(&uav_orig);
        }

        uav.update();
        ufo.update();

        thread::sleep(one_second - start.elapsed());

//        break;
    }
}

fn inet_producer(_channel: &mpsc::Receiver<Vec<u8>>) -> () {
    let mut uav = CwithV::new();
    let mut ufo = CwithV::new();

    let mut ouraddress = String::new();
    let mut dstaddress = String::new();

    for argument in env::args() {
        if argument.starts_with("-uav=") {
            uav.set_cli(&argument[5..]);
        }
        if argument.starts_with("-ufo=") {
            ufo.set_cli(&argument[5..]);
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
        }
    }

    let socket: UdpSocket;
    match UdpSocket::bind(ouraddress) {
        Ok(nn) => socket = nn,
        Err(e) => {println!("Error: {}", e); return;},
    }

    let one_second = time::Duration::new(1, 0);

    let mut datastreamrequest   = mavlink::msg66::Message::new();
    let mut status              = mavlink::msg203::Message::new();
    let mut ownship             = mavlink::msg202::Message::new();
    let mut trafficreport       = mavlink::msg246::Message::new();

    trafficreport.icao = 0x00300100;

    loop {
        let start = time::Instant::now();

        for msgid in [66, 203, 202, 246].iter() {
            match
                match *msgid {
                    66 =>  {
                        socket.send_to(datastreamrequest.serialise().message(), &dstaddress)
                    },
                    203 =>  {
                        socket.send_to(status.serialise().message(), &dstaddress)
                    },
                    202 =>  {
                        socket.send_to(ownship.set_candv(&uav).serialise().message(), &dstaddress)
                    },
                    246 =>  {
                        socket.send_to(trafficreport.set_candv(&ufo).serialise().message(), &dstaddress)
                    },
                    _  =>  {
                        Err(Error::new(ErrorKind::Other, format!("WTF: msgid = {}", msgid)))
                    },
                }
            {
                Ok(_) => (),
                Err(e) => {println!("Error: {}", e); return;},
            }
        }

        uav.update();
        ufo.update();

        thread::sleep(one_second - start.elapsed());

//        break;
    }
}

// EOF
