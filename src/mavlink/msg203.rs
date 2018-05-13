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
//! by the ADS-B device being simulated but the receiver ignores it.
//!
use std::io::{Error, ErrorKind};

use mavlink;
use mavlink::byteorder::{WriteBytesExt};

// ---------------------------------------------------------------------------

/// The length of MAVLink 203 messages in bytes
const MSGLEN: usize = msglen!(1);

// ---------------------------------------------------------------------------

/// The MAVLink 203 message structure
pub struct Message {
    buffy: [u8; MSGLEN],

    pub status:     u8,
}

// ---------------------------------------------------------------------------

/// The implementation of methods for the MAVLink 203 message tyoe
impl Message {
    // new() creates and initialises a MAVLink 203 message structure
    pub fn new() -> Message {
        Message {
            buffy: [0; MSGLEN],

            status: 1,
        }
    }
}

// ---------------------------------------------------------------------------

/// The implementation of the MAVLink message traits for the 203 message type
impl mavlink::Message for Message {
    const MSGID: u8 = 203;
    const EXTRA: u8 = 0x55;
    const PAYLEN: usize = paylen!(MSGLEN);

    // message() returns the message byte array (for trait use only)
    fn message(&mut self) -> &mut [u8] {
        &mut self.buffy
    }

    // pack_payload() implements the MAVLink message serialise() trait
    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        buffy.write_u8(self.status)?;

        Ok(())
    }

    // unpack_payload() implements the MAVLink message deserialise() trait
    fn unpack_payload(&mut self) -> Result<(),Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}

// EOF
