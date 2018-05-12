//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The msg203 module implements the _mavlink message trait_ for the
//! MAVLink 'status' message (id 203).
//!
//! Only message serialise is actually implemented
//! as the ADS-B Simulator need only generate this message.
//!
//! No getter/setter functions are implemented:  the message is generated
//! by the ADS-B device being simulated but the receiver ignore it.
//!
use std::io::{Error, ErrorKind};

use mavlink;
use mavlink::byteorder::{WriteBytesExt};

// ---------------------------------------------------------------------

/*
 * Avionix 203 'status' message (in)
 */

const MSGLEN: usize = msglen!(1);

pub struct Message {
    buffy: [u8; MSGLEN],

    pub status:     u8,
}

impl Message {
    pub fn new() -> Message {
        Message {
            buffy: [0; MSGLEN],

            status: 1,
        }
    }
}

impl mavlink::Message for Message {
    const MSGID: u8 = 203;
    const EXTRA: u8 = 0x55;
    const PAYLEN: usize = paylen!(MSGLEN);

    fn dump(&self) -> &Self {
        Self::dump_message (&self.buffy);

        self
    }

    fn message(&mut self) -> &mut [u8] {
        &mut self.buffy
    }

    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        buffy.write_u8(self.status)?;

        Ok(())
    }

    fn unpack_payload(&mut self) -> Result<(),Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}

// EOF
