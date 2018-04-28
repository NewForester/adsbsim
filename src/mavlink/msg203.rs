//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
use std::io::Error;

use mavlink;
use mavlink::byteorder::{WriteBytesExt};

// ---------------------------------------------------------------------

/*
 * Avionix 203 'status' message (in)
 */

pub struct Message {
    buffy: Vec<u8>,

    pub status:     u8,
}

impl Message {
    pub fn new() -> Message {
        Message {
            buffy: Vec::new(),
            status: 1,
        }
    }
}

impl mavlink::Message for Message {
    const MSGID: u8 = 203;
    const PAYLEN: u8 = 1;
    const EXTRA: u8 = 0x55;

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
        buffy.write_u8(self.status)?;

        Ok(())
    }
}

// EOF
