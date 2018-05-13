//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The mavlink module declares a trait to ensure MAVLink messages are handled
//! consistently and provide DRY code as appropriate.
//!
//! The two most important traits are:
//!
//!  * serialise - message ready for transmission
//!  * deserialise - on arrival so message payload is available for processing
//!
//! Message specific packing/unpacking is delegated to modules implementing
//! these traits.
//!
extern crate byteorder;
extern crate crc16;

use std::io::Error;

use self::byteorder::{LittleEndian, WriteBytesExt};

use mavlink;

// ---------------------------------------------------------------------------

#[macro_export]
/// msglen!() returns a MAVLink message length given its payload length
macro_rules! msglen {
    ($paylen:expr) => (
        $paylen + mavlink::HDR_SIZE + mavlink::CRC_SIZE
    )
}

#[macro_export]
// paylen!() returns the payload length given a MAVLink message length
macro_rules! paylen {
    ($msglen:expr) => (
        $msglen - mavlink::HDR_SIZE - mavlink::CRC_SIZE
    )
}

// ---------------------------------------------------------------------------

/// The sizes, in bytes. of the MAVLink message header and checksum, respectively
const HDR_SIZE: usize = 6;
const CRC_SIZE: usize = 2;

/// The offset of the payload within a MAVLink message
const PAYLOAD:  usize = HDR_SIZE;

// ---------------------------------------------------------------------------

/// The unique sequence number used in the header of all published messages
static mut SEQNO_OUT: u8 = 0;

// ---------------------------------------------------------------------------

/// The MAVLink message header structure
struct Header {
    pub mavstx:     u8,
    pub paylen:     u8,
    pub seqno:      u8,
    pub sysid:      u8,
    pub compid:     u8,
    pub msgid:      u8,
}

// ---------------------------------------------------------------------------

/// The definition and partial implementation of the MAVLink message traits
pub trait Message {
    const MSGID: u8;
    const EXTRA: u8;
    const PAYLEN: usize;

    // dump() prints the message byte array (for debugging use only)
    fn dump(&mut self) -> &Self {
        print!("Dump message {:3}:", Self::MSGID);
        for ii in self.message() {
            print!(" {:02x}", ii);
        }
        println!();

        self
    }

    // message() returns the message byte array (for internal use only)
    fn message(&mut self) -> &mut [u8];

    // the serialise() trait converts a MAVLink message type into a byte array
    fn serialise(&mut self) -> &[u8] {
        let mut buffy: Vec<u8> = Vec::with_capacity(msglen!(Self::PAYLEN));

        match Self::pack_message(self, &mut buffy) {
            Ok(_)  => {
                let message = self.message();

                for ii in 0 .. msglen!(Self::PAYLEN) {
                    message[ii] = buffy[ii];
                }
            }
            Err(_) => {
                println!("Serialisation of message {} failed", Self::MSGID);

                let message = self.message();

                for ii in 0 .. msglen!(Self::PAYLEN) {
                    message[ii] = 0;
                }
            }
        }

        self.message()
    }

    // the deserialise() trait converts a byte array into a MAVLink message type
    fn deserialise(&mut self, buffy: &Vec<u8>) -> &mut Self {
        {
            let message = self.message();

            for ii in 0 .. msglen!(Self::PAYLEN) {
                message[ii] = buffy[ii];
            }
        }

        match self.unpack_message() {
            Ok(_)  => {
                ();
            }
            Err(_) => {
                println!("Deserialisation of message {} failed", Self::MSGID);
            }
        }

        self
    }

    // pack_message() implements the MAVLink message serialise() trait
    fn pack_message(&mut self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        Self::pack_header(buffy)?;

        self.pack_payload(buffy)?;

        Self::pack_crc(buffy)?;

        Ok(())
    }

    // pack_header() serialises the MAVLink message header
    fn pack_header(buffy: &mut Vec<u8>) -> Result<(),Error> {
        let mut header = Header {
            mavstx:     0xfe,
            paylen:     Self::PAYLEN as u8,
            seqno:      0,
            sysid:      0x19,
            compid:     0x59,
            msgid:      Self::MSGID,
        };

        unsafe {header.seqno = SEQNO_OUT;}

        buffy.write_u8(header.mavstx)?;
        buffy.write_u8(header.paylen)?;
        buffy.write_u8(header.seqno)?;
        buffy.write_u8(header.sysid)?;
        buffy.write_u8(header.compid)?;
        buffy.write_u8(header.msgid)?;

        unsafe {SEQNO_OUT = SEQNO_OUT.wrapping_add(1);}

        Ok(())
    }

    // pack_payload() serialises a MAVLink message payload
    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error>;

    // pack_header() calculates and serialises the MAVLink message checksum
    fn pack_crc(buffy: &mut Vec<u8>) -> Result<(),Error> {
        let mut crc = crc16::State::<crc16::MCRF4XX>::new();

        crc.update(&buffy[1..]);
        crc.update(&[Self::EXTRA]);

        buffy.write_u16::<LittleEndian>(crc.get())?;

        Ok(())
    }

    // unpack_payload() implements the MAVLink message deserialise() trait
    fn unpack_message(&mut self) -> Result<(),Error> {
//        Self::unpack_crc(self.message())?;

//        Self::unpack_header(self.message())?;

        self.unpack_payload()?;

        Ok(())
    }

    // unpack_payload() deserialises a MAVLink message payload
    fn unpack_payload(&mut self) -> Result<(),Error>;
}

// ---------------------------------------------------------------------------

// MAVLink message implementations
pub mod msg66;
pub mod msg84;
pub mod msg202;
pub mod msg203;
pub mod msg246;

// EOF
