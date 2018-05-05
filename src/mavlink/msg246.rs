//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
use std::io::{Write, Error, ErrorKind};

use ::coords::CwithV;

use mavlink;
use mavlink::byteorder::{LittleEndian, WriteBytesExt};

// ---------------------------------------------------------------------

/*
 * Avionix 246 'traffic report' message (in)
 */

pub struct Message {
    buffy: Vec<u8>,

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

impl Message {
    pub fn new() -> Message {
        let mut safe = Message {
            buffy: Vec::new(),

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

    pub fn set_gps(&mut self, latitude: f32, longitude: f32) -> &mut Self {
        self.lat = (latitude * 1.0e7) as i32;
        self.lon = (longitude * 1.0e7) as i32;
        self.validflags |= VF::LatLonValid as u16;

        self
    }

    pub fn set_altitude(&mut self, altitude: f32) -> &mut Self {
        self.altitude = (altitude * 1.0e3) as i32;
        self.validflags |= VF::AltitudeValid as u16;

        self
    }
    pub fn set_rateofclimb(&mut self, updown_velocity: f32) -> &mut Self {
        self.vervelocity = (updown_velocity * 1.0e2) as i16;
        self.validflags |= VF::VerticalVelocityValid as u16;

        self
    }

    pub fn set_heading(&mut self, heading: f32) -> &mut Self {

        self.heading = (heading * 1.0e2) as i16;
        self.validflags |= VF::HeadingValid as u16;

        self
    }
    pub fn set_groundspeed(&mut self, speed: f32) -> &mut Self {
        self.horvelocity = (speed* 1.0e2) as u16;
        self.validflags |= VF::VelocityValid as u16;

        self
    }

    pub fn set_candv(&mut self, candv: &CwithV) -> &mut Self {
        self.validflags &= VF::CallsignValid as u16;

        self.set_gps(candv.get_latitude(), candv.get_longitude());

        self.set_altitude(candv.get_altitude());
        self.set_rateofclimb(candv.get_rateofclimb());

        self.set_heading(candv.get_heading());
        self.set_groundspeed(candv.get_groundspeed());

        self
    }
}

impl mavlink::Message for Message {
    const MSGID: u8 = 246;
    const EXTRA: u8 = 0xb8;
    const PAYLEN: usize = 38;

    fn serialise(&mut self) -> &mut Self {
        self.buffy = Self::serialise_message(self);

        self
    }

    fn dump(&self) -> &Self {
        Self::dump_message (&self.buffy);

        self
    }

    fn message(&self) -> &[u8] {
        &self.buffy
    }

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

    fn unpack_payload(&mut self, mut _payload: &[u8]) -> Result<(),Error> {
       Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}

// EOF
