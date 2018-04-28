//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
use std::env;
use std::{thread, time};
use std::net::UdpSocket;
use std::io::{Error, ErrorKind};

mod coords;
use coords::CwithV;

mod mavlink;
use mavlink::Message;

fn main () -> () {
    let mut ouraddress = String::new();
    let mut dstaddress = String::new();

    let mut uav = CwithV::new();
    let mut ufo = CwithV::new();

    for argument in env::args() {
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
        if argument.starts_with("-uav=") {
            uav.set_cli(&argument[5..]);
        }
        if argument.starts_with("-ufo=") {
            ufo.set_cli(&argument[5..]);
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
    }
}

// EOF
