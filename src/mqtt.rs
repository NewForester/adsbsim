//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
extern crate mosquitto_client;

// ---------------------------------------------------------------------

/*
 * Simple MQTT client implementation
 */

#[derive(Clone)]
pub struct Client {
    clientid:   String,
    host:       String,
    port:       u32,

    pub pubtopic:   String,
    pub subtopic:   String,

    handle:     mosquitto_client::Mosquitto,
}

impl Client {
    pub fn new() -> Client {
        Client {
            clientid:   String::new(),
            host:       "127.0.0.1".to_string(),
            port:       1883,
            pubtopic:   String::new(),
            subtopic:   String::new(),
            handle:     mosquitto_client::Mosquitto::new("")
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) -> &Self {
        println!("clientid: {}", self.clientid);
        println!("host:     {}", self.host);
        println!("port:     {}", self.port);
        println!("pubtopic: {}", self.pubtopic);
        println!("subtopic: {}", self.subtopic);

        self
    }

    pub fn set_cli(&mut self, cli: &str) -> &mut Self {
        let fission: Vec<&str> = cli.split(',').collect();

        if fission[0].len() != 0 {
            self.clientid = fission[0].to_string();
        }

        if fission.len() > 1 && fission[1].len() != 0 {
            let pair: Vec<&str> = fission[1].split(':').collect();

            self.host = pair[0].to_string();

            if pair.len() > 1 {
                self.port = pair[1].parse().unwrap();
            }
        }

        if fission.len() > 2 && fission[2].len() != 0 {
            let pair: Vec<&str> = fission[2].split(':').collect();

            self.pubtopic = pair[0].to_string();

            if pair.len() > 1 {
                self.subtopic = pair[1].to_string();
            }
        }

        self
    }

    pub fn connect(&mut self) -> &mut Self {
        self.handle = mosquitto_client::Mosquitto::new(&self.clientid);

        match self.handle.connect(&self.host, self.port) {
            Ok(_)  => println!("MQTT connection successful"),
            Err(e) => println!("MQTT connection error: {}", e),
        }

        self
    }

    pub fn disconnect(&mut self) -> &mut Self {
        match self.handle.loop_until_disconnect(-1) {
            Ok(_)  => println!("MQTT wait for disconnect successful"),
            Err(e) => println!("MQTT wait for disconnect error: {}", e),
        }

        match self.handle.disconnect() {
            Ok(_)  => println!("MQTT disconnection successful"),
            Err(e) => println!("MQTT disconnection error: {}", e),
        }

        self
    }

    pub fn publish(&mut self, payload: &[u8]) -> &mut Self {
        match self.handle.publish(&self.pubtopic, payload, 0, false) {
            Ok(_)  => (),
            Err(e) => println!("MQTT publish error: {}", e),
        }

        self
    }

    pub fn subscribe<F>(&mut self, callback: F) -> &mut Self
                where F: Fn(&[u8]) -> () {
        match self.handle.subscribe(&self.subtopic, 1) {
            Ok(_)  => println!("MQTT subscribe successful"),
            Err(e) => println!("MQTT subscribe error: {}", e),
        }

        {
            let mut mc = self.handle.callbacks(0);

            mc.on_message(move |data, msg| {
                *data += 1;
                callback(msg.payload());
            });

            match self.handle.loop_until_disconnect(200) {
                Ok(_)  => println!("MQTT disconnect successful"),
                Err(e) => println!("MQTT disconnect error: {}", e),
            }
            println!("received {} messages",mc.data);
        }

        self
    }
}

// EOF