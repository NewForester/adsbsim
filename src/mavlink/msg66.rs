//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
use std::io::Error;

use mavlink;
use mavlink::byteorder::{WriteBytesExt};

// ---------------------------------------------------------------------

/*
 * MAVLink 66 'datastream request' message (obsolete but still comes in)
 */

pub struct Message {
    buffy: Vec<u8>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            buffy: Vec::new(),
        }
    }
}

impl mavlink::Message for Message {
    const MSGID: u8 = 66;
    const PAYLEN: u8 = 6;
    const EXTRA: u8 = 0x94;

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
        for _ii in 0..6 {
            buffy.write_u8(0)?;
        }

        Ok(())
    }
}

// EOF
