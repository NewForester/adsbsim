//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The msg84 module implements the _mavlink message trait_ for the
//! MAVLink 'set target position local ned' message (id 84).
//!
//! Both message serialise and deserialise are implemented
//! although in the ADS-B Simulator only deserialise is used.
//!
//! No getter/setter functions are implemented:  the MAVLink 84 message uses
//! a Cartesian frame of reference with a floating point representation.
//!
use std::io::{Error};

use mavlink;
use mavlink::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

// ---------------------------------------------------------------------

/*
 * Avionix 84 'set position target local ned' message (in)
 */

const MSGLEN: usize = msglen!(53);

pub struct Message {
    buffy: [u8; MSGLEN],

    pub time_boot_ms:       u32,
    pub system_id:          u8,
    pub component_id:       u8,
    pub coordinate_frame:   u8,
    pub type_mask:          u16,
    pub x:                  f32,
    pub y:                  f32,
    pub z:                  f32,
    pub vx:                 f32,
    pub vy:                 f32,
    pub vz:                 f32,
    pub afx:                f32,
    pub afy:                f32,
    pub afz:                f32,
    pub yaw:                f32,
    pub yaw_rate:           f32,
}

impl Message {
    pub fn new() -> Message {
        let safe = Message {
            buffy: [0; MSGLEN],

            time_boot_ms:       0,
            system_id:          0,
            component_id:       0,
            coordinate_frame:   0x0,
            type_mask:          0,
            x:                  0.0,
            y:                  0.0,
            z:                  0.0,
            vx:                 0.0,
            vy:                 0.0,
            vz:                 0.0,
            afx:                0.0,
            afy:                0.0,
            afz:                0.0,
            yaw:                0.0,
            yaw_rate:           0.0,
        };

        safe
    }
}

impl mavlink::Message for Message {
    const MSGID: u8 = 246;
    const EXTRA: u8 = 0xb8;
    const PAYLEN: usize = paylen!(MSGLEN);

    fn dump(&self) -> &Self {
        Self::dump_message (&self.buffy);

        self
    }

    fn message(&mut self) -> &mut [u8] {
        &mut self.buffy
    }

    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        buffy.write_u32::<LittleEndian>(self.time_boot_ms)?;

        buffy.write_f32::<LittleEndian>(self.x)?;
        buffy.write_f32::<LittleEndian>(self.y)?;
        buffy.write_f32::<LittleEndian>(self.z)?;

        buffy.write_f32::<LittleEndian>(self.vx)?;
        buffy.write_f32::<LittleEndian>(self.vy)?;
        buffy.write_f32::<LittleEndian>(self.vz)?;

        buffy.write_f32::<LittleEndian>(self.afx)?;
        buffy.write_f32::<LittleEndian>(self.afy)?;
        buffy.write_f32::<LittleEndian>(self.afz)?;

        buffy.write_f32::<LittleEndian>(self.yaw)?;
        buffy.write_f32::<LittleEndian>(self.yaw_rate)?;

        buffy.write_u16::<LittleEndian>(self.type_mask)?;

        buffy.write_u8(self.system_id)?;
        buffy.write_u8(self.component_id)?;
        buffy.write_u8(self.coordinate_frame)?;

        Ok(())
    }

    fn unpack_payload(&mut self) -> Result<(),Error> {
        let mut payload = &self.buffy[mavlink::PAYLOAD..];

        self.time_boot_ms = payload.read_u32::<LittleEndian>()?;

        self.x = payload.read_f32::<LittleEndian>()?;
        self.y = payload.read_f32::<LittleEndian>()?;
        self.z = payload.read_f32::<LittleEndian>()?;

        self.vx = payload.read_f32::<LittleEndian>()?;
        self.vy = payload.read_f32::<LittleEndian>()?;
        self.vz = payload.read_f32::<LittleEndian>()?;

        self.afx = payload.read_f32::<LittleEndian>()?;
        self.afy = payload.read_f32::<LittleEndian>()?;
        self.afz = payload.read_f32::<LittleEndian>()?;

        self.yaw = payload.read_f32::<LittleEndian>()?;
        self.yaw_rate = payload.read_f32::<LittleEndian>()?;

        self.type_mask = payload.read_u16::<LittleEndian>()?;

        self.system_id = payload.read_u8()?;
        self.component_id = payload.read_u8()?;
        self.coordinate_frame = payload.read_u8()?;

        Ok(())
    }
}

// EOF
