//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The msg246 module implements the _mavlink message trait_ for the
//! MAVLink 'traffic report' message (id 246).
//!
//! Only message serialise is actually implemented
//! as the ADS-B Simulator need only generate this message.
//!
//! A number of setter functions are implemented to support this.
//! Setting message fields without using these functions is not recommended.
//!
//! Although these functions are all declared `pub`,
//! the  ADS-B Simulator uses only `set_cwithv()` 'publicly'.
//!
//! All message fields are `pub` so direct access is possible but check that
//! such access is safe before doing so and considering implementing an
//! appropriate getter/setter function.
//!
use std::io::{Write, Error, ErrorKind};

use ::coords::CwithV;

use mavlink;
use mavlink::byteorder::{LittleEndian, WriteBytesExt};

// ---------------------------------------------------------------------------

/// The length of MAVLink 246 messages in bytes
const MSGLEN: usize = msglen!(38);

// ---------------------------------------------------------------------------

/// The MAVLink 246 message structure
pub struct Message {
    buffy: [u8; MSGLEN],

    pub icao:           u32,
    pub lat:            i32,
    pub lon:            i32,
    pub altitude:       i32,
    pub heading:        i16,
    pub horvelocity:    u16,
    pub vervelocity:    i16,
    pub validflags:     u16,
    pub squawk:         u16,
    pub altitudetype:   u8,
    pub callsign:       [u8; 9],
    pub emittertype:    u8,
    pub tslc:           u8,
}

// ---------------------------------------------------------------------------

/// Names for pertinent flags in the validflags field of the MAVLink 246 message
enum VF {
    LatLonValid             = 0x0001,
    AltitudeValid           = 0x0002,
    HeadingValid            = 0x0004,
    VelocityValid           = 0x0008,
    CallsignValid           = 0x0010,
//  IDENT_VALID             = 0x0020,
//  SIMULATED_REPORT        = 0x0040,
    VerticalVelocityValid   = 0x0080,
//  BARO_VALID              = 0x0100,
//  SOURCE UAT              = 0x8000,
}

// ---------------------------------------------------------------------------

/// The implementation of methods for the MAVLink 246 message type
impl Message {
    // new() creates and initialises a MAVLink 246 message structure
    pub fn new() -> Message {
        let mut safe = Message {
            buffy: [0; MSGLEN],

            icao:           0,
            lat:            0,
            lon:            0,
            altitude:       0,
            heading:        0,
            horvelocity:    0,
            vervelocity:    0,
            validflags:     0,
            squawk:         0xdead,
            altitudetype:   0,
            callsign:       [0, 0, 0, 0, 0, 0, 0, 0, 0],
            emittertype:    0,
            tslc:           1,
        };

        safe.set_callsign("D-RisQ");

        safe
    }

    // set_callsign() sets the message callsign from the given string safely
    pub fn set_callsign(&mut self, callsign: &str) -> &mut Self {
        let safe = String::from(callsign).into_bytes();

        for ii in 0 .. safe.len() {
            self.callsign[ii] = safe[ii];
        }
        for ii in safe.len() .. self.callsign.len() {
            self.callsign[ii] = 0;
        }
        self.validflags |= VF::CallsignValid as u16;

        self
    }

    // set_gps() sets the message latitude and longitude (converting floats to a scaled integers)
    pub fn set_gps(&mut self, latitude: f32, longitude: f32) -> &mut Self {
        self.lat = (latitude * 1.0e7) as i32;
        self.lon = (longitude * 1.0e7) as i32;
        self.validflags |= VF::LatLonValid as u16;

        self
    }
    // set_altitude() sets the message altitude converting (a float to a scaled integer)
    pub fn set_altitude(&mut self, altitude: f32) -> &mut Self {
        self.altitude = (altitude * 1.0e3) as i32;
        self.validflags |= VF::AltitudeValid as u16;

        self
    }

    // set_rateofclimb() sets the message 'vertical' velocity (converting a float to a scaled integer)
    pub fn set_rateofclimb(&mut self, updown_velocity: f32) -> &mut Self {
        self.vervelocity = (updown_velocity * 1.0e2) as i16;
        self.validflags |= VF::VerticalVelocityValid as u16;

        self
    }
    // set_heading() sets the message over-the-ground heading (converting a float to a scaled integer)
    pub fn set_heading(&mut self, heading: f32) -> &mut Self {

        self.heading = (heading * 1.0e2) as i16;
        self.validflags |= VF::HeadingValid as u16;

        self
    }
    // set_groundspeed() sets the message over-the-ground speed (converting a float to a scaled integer)
    pub fn set_groundspeed(&mut self, speed: f32) -> &mut Self {
        self.horvelocity = (speed* 1.0e2) as u16;
        self.validflags |= VF::VelocityValid as u16;

        self
    }

    // set_cwithv() sets message position and velocity from those held in the given CwithV structure
    pub fn set_cwithv(&mut self, cwithv: &CwithV) -> &mut Self {
        self.validflags &= VF::CallsignValid as u16;

        self.set_gps(cwithv.get_latitude(), cwithv.get_longitude());

        self.set_altitude(cwithv.get_altitude());
        self.set_rateofclimb(cwithv.get_rateofclimb());

        self.set_heading(cwithv.get_heading());
        self.set_groundspeed(cwithv.get_groundspeed());

        self
    }
}

// ---------------------------------------------------------------------------

/// The implementation of the MAVLink message traits for the 246 message type
impl mavlink::Message for Message {
    const MSGID: u8 = 246;
    const EXTRA: u8 = 0xb8;
    const PAYLEN: usize = paylen!(MSGLEN);

    // message() returns the message byte array (for trait use only)
    fn message(&mut self) -> &mut [u8] {
        &mut self.buffy
    }

    // pack_payload() implements the MAVLink message serialise() trait
    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        buffy.write_u32::<LittleEndian>(self.icao)?;

        buffy.write_i32::<LittleEndian>(self.lat)?;
        buffy.write_i32::<LittleEndian>(self.lon)?;
        buffy.write_i32::<LittleEndian>(self.altitude)?;

        buffy.write_i16::<LittleEndian>(self.heading)?;
        buffy.write_u16::<LittleEndian>(self.horvelocity)?;
        buffy.write_i16::<LittleEndian>(self.vervelocity)?;

        buffy.write_u16::<LittleEndian>(self.validflags)?;
        buffy.write_u16::<LittleEndian>(self.squawk)?;

        buffy.write_u8(self.altitudetype)?;
        buffy.write(&self.callsign)?;
        buffy.write_u8(self.emittertype)?;
        buffy.write_u8(self.tslc)?;

        Ok(())
    }

    // unpack_payload() implements the MAVLink message deserialise() trait
    fn unpack_payload(&mut self) -> Result<(),Error> {
       Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}

// EOF
