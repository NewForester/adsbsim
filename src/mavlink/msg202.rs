//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The msg202 module implements the _mavlink message trait_ for the
//! MAVLink 'ownship' message (id 202).
//!
//! Both message serialise and deserialise are implemented.
//!
//! A number of setter/getter functions are implemented to support this.
//! These convert between scaled integer and floating point representations.
//! Their use is recommended.
//!
//! Although these functions are declared `pub`, the  ADS-B Simulator
//! only uses `set_cwithv()` and  `get_cwithv()` 'publicly'.
//!
//! All message fields are `pub` so direct access is possible but check that
//! such access is safe before doing so and considering implementing an
//! appropriate getter/setter function.
//!
extern crate chrono;

use self::chrono::Utc;

use std::io::{Error};

use ::coords::CwithV;

use mavlink;
use mavlink::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

// ---------------------------------------------------------------------------

/// The length of MAVLink 202 messages in bytes
const MSGLEN: usize = msglen!(42);

// ---------------------------------------------------------------------------

/// The MAVLink 202 message structure
pub struct Message {
    buffy: [u8; MSGLEN],

    pub utctime:    u32,
    pub latitude:   i32,
    pub longitude:  i32,
    pub altpres:    i32,
    pub altgnss:    i32,
    pub acchoriz:   u32,
    pub accvert:    u16,
    pub accvel:     u16,
    pub velvert:    i16,
    pub nsvog:      i16,
    pub ewvog:      i16,
    pub state:      u16,
    pub squawk:     u16,
    pub fixtype:    u8,
    pub numsats:    u8,
    pub emstatus:   u8,
    pub control:    u8,
}

// ---------------------------------------------------------------------------

/// The implementation of methods for the MAVLink 202 message type
impl Message {
    // new() creates and initialises a MAVLink 202 message structure
    pub fn new() -> Message {
        Message {
            buffy: [0; MSGLEN],

            utctime:    0,
            latitude:   0,
            longitude:  0,
            altpres:    0,
            altgnss:    0,
            acchoriz:   0,
            accvert:    0,
            accvel:     0,
            velvert:    0,
            nsvog:      0,
            ewvog:      0,
            state:      0,
            squawk:     0xdead,
            fixtype:    4,
            numsats:    7,
            emstatus:   0,
            control:    0x03,
        }
    }

    // set_latitude() sets the message latitude (converting a float to a scaled integer)
    pub fn set_latitude(&mut self, latitude: f32) -> &mut Self {
        self.latitude = (latitude * 1.0e7) as i32;

        self
    }
    // set_longitude() sets the message longitude (converting a float to a scaled integer)
    pub fn set_longitude(&mut self, longitude: f32) -> &mut Self {
        self.longitude = (longitude * 1.0e7) as i32;

        self
    }
    // set_altitude() sets the message altitude (converting a float to a scaled integer)
    pub fn set_altitude(&mut self, altitude: f32) -> &mut Self {
        self.altpres = (altitude * 1.0e3) as i32;

        self
    }

    // set_rateofclimb() sets the message 'vertical' velocity (converting a float to a scaled integer)
    pub fn set_rateofclimb(&mut self, updown_velocity: f32) -> &mut Self {
        self.velvert = (updown_velocity * 1.0e2) as i16;

        self
    }
    // set_ns_velocity() sets the message 'horizontal' north/south (converting a float to a scaled integer)
    pub fn set_ns_velocity(&mut self, northsouth_velocity: f32) -> &mut Self {
        self.nsvog = (northsouth_velocity * 1.0e2) as i16;

        self
    }
    // set_ew_velocity() sets the message 'horizontal' east/west (converting a float to a scaled integer)
    pub fn set_ew_velocity(&mut self, eastwest_velocity: f32) -> &mut Self {
        self.ewvog = (eastwest_velocity * 1.0e2) as i16;

        self
    }

    // set_cwithv() sets message position and velocity from those held in the given CwithV structure
    pub fn set_cwithv(&mut self, cwithv: &CwithV) -> &mut Self {
        self.set_latitude(cwithv.get_latitude());
        self.set_longitude(cwithv.get_longitude());
        self.set_altitude(cwithv.get_altitude());

        self.set_ns_velocity(cwithv.get_ns_velocity());
        self.set_ew_velocity(cwithv.get_ew_velocity());
        self.set_rateofclimb(cwithv.get_rateofclimb());

        self
    }

    // get_latitude() returns the message latitude (converting a scaled integer to float point)
    pub fn get_latitude(&mut self) -> f32 {
        self.latitude as f32 / 1.0e7
    }
    // get_longitude() returns the message longitude (converting a scaled integer to float point)
    pub fn get_longitude(&mut self) -> f32 {
        self.longitude as f32 / 1.0e7
    }
    // get_altitude() returns the message altitude (converting a scaled integer to float point)
    pub fn get_altitude(&mut self) -> f32 {
        self.altpres as f32 / 1.0e3
    }

    // get_rateofclimb() returns the message 'vertical' velocity (converting a scaled integer to float point)
    pub fn get_rateofclimb(&mut self) -> f32 {
        self.velvert as f32 / 1.0e2
    }
    // get_ns_velocity() returns the message 'horizontal' north/south (converting a scaled integer to float point)
    pub fn get_ns_velocity(&mut self) -> f32 {
        self.nsvog as f32 / 1.0e2
    }
    // get_ew_velocity() returns the message 'horizontal' east/west (converting a scaled integer to float point)
    pub fn get_ew_velocity(&mut self) -> f32 {
        self.ewvog as f32 / 1.0e2
    }

    // get_cwithv() sets the given CwithV structure to the message position and velocity
    pub fn get_cwithv(&mut self, cwithv: &mut CwithV) -> &mut Self {
        cwithv.set_position(self.get_latitude(), self.get_longitude(), self.get_altitude());

        cwithv.set_velocity(self.get_ns_velocity(), self.get_ew_velocity(), self.get_rateofclimb());

        self
    }
}

// ---------------------------------------------------------------------------

/// The implementation of the MAVLink message traits for the 202 message type
impl mavlink::Message for Message {
    const MSGID: u8 = 202;
    const EXTRA: u8 = 0x07;
    const PAYLEN: usize = paylen!(MSGLEN);

    // message() returns the message byte array (for trait use only)
    fn message(&mut self) -> &mut [u8] {
        &mut self.buffy
    }

    // pack_payload() implements the MAVLink message serialise() trait
    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        buffy.write_u32::<LittleEndian>(Utc::now().timestamp() as u32)?;

        buffy.write_i32::<LittleEndian>(self.latitude)?;
        buffy.write_i32::<LittleEndian>(self.longitude)?;
        buffy.write_i32::<LittleEndian>(self.altpres)?;
        buffy.write_i32::<LittleEndian>(self.altgnss)?;

        buffy.write_u32::<LittleEndian>(self.acchoriz)?;
        buffy.write_u16::<LittleEndian>(self.accvert)?;
        buffy.write_u16::<LittleEndian>(self.accvel)?;

        buffy.write_i16::<LittleEndian>(self.velvert)?;
        buffy.write_i16::<LittleEndian>(self.nsvog)?;
        buffy.write_i16::<LittleEndian>(self.ewvog)?;

        buffy.write_u16::<LittleEndian>(self.state)?;
        buffy.write_u16::<LittleEndian>(self.squawk)?;

        buffy.write_u8(self.fixtype)?;
        buffy.write_u8(self.numsats)?;
        buffy.write_u8(self.emstatus)?;
        buffy.write_u8(self.control)?;

        Ok(())
    }

    // unpack_payload() implements the MAVLink message deserialise() trait
    fn unpack_payload(&mut self) -> Result<(),Error> {
        let mut payload = &self.buffy[mavlink::PAYLOAD..];

        self.utctime = payload.read_u32::<LittleEndian>()?;

        self.latitude = payload.read_i32::<LittleEndian>()?;
        self.longitude = payload.read_i32::<LittleEndian>()?;
        self.altpres = payload.read_i32::<LittleEndian>()?;
        self.altgnss = payload.read_i32::<LittleEndian>()?;

        self.acchoriz = payload.read_u32::<LittleEndian>()?;
        self.accvert = payload.read_u16::<LittleEndian>()?;
        self.accvel = payload.read_u16::<LittleEndian>()?;

        self.velvert = payload.read_i16::<LittleEndian>()?;
        self.nsvog = payload.read_i16::<LittleEndian>()?;
        self.ewvog = payload.read_i16::<LittleEndian>()?;

        self.state = payload.read_u16::<LittleEndian>()?;
        self.squawk = payload.read_u16::<LittleEndian>()?;

        self.fixtype = payload.read_u8()?;
        self.numsats = payload.read_u8()?;
        self.emstatus = payload.read_u8()?;
        self.control = payload.read_u8()?;

        Ok(())
    }
}

// EOF
