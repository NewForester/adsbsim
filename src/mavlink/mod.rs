//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The mavlink module declares a trait to ensure MAVLink messages are handled
//! consistently and provide DRY code as appropriate.
//!
//! The two most important traits are:
//!
//!  * serialise - message payload ready for transmission
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

const HDR_SIZE: usize = 6;
const CRC_SIZE: usize = 2;

#[macro_export]
macro_rules! msglen {
    ($paylen:expr) => (
        $paylen + mavlink::HDR_SIZE + mavlink::CRC_SIZE
    )
}

#[macro_export]
macro_rules! paylen {
    ($msglen:expr) => (
        $msglen - mavlink::HDR_SIZE - mavlink::CRC_SIZE
    )
}

const PAYLOAD: usize = HDR_SIZE;

static mut SEQNO_OUT: u8 = 0;

struct Header {
    pub mavstx:     u8,
    pub paylen:     u8,
    pub seqno:      u8,
    pub sysid:      u8,
    pub compid:     u8,
    pub msgid:      u8,
}

pub trait Message {
    const MSGID: u8;
    const EXTRA: u8;
    const PAYLEN: usize;

    fn dump(&self) -> &Self;

    fn message(&mut self) -> &mut [u8];

    fn dump_message(message: &[u8]) -> () {
        print!("Dump message {:3}:", Self::MSGID);
        for ii in message {
            print!(" {:02x}", ii);
        }
        println!();
    }

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

    fn pack_message(&mut self, buffy: &mut Vec<u8>) -> Result<(),Error> {
        Self::pack_header(buffy)?;

        self.pack_payload(buffy)?;

        Self::pack_crc(buffy)?;

        Ok(())
    }

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

    fn pack_payload(&self, buffy: &mut Vec<u8>) -> Result<(),Error>;

    fn pack_crc(buffy: &mut Vec<u8>) -> Result<(),Error> {
        let mut crc = crc16::State::<crc16::MCRF4XX>::new();

        crc.update(&buffy[1..]);
        crc.update(&[Self::EXTRA]);

        buffy.write_u16::<LittleEndian>(crc.get())?;

        Ok(())
    }

    fn unpack_message(&mut self) -> Result<(),Error> {
//        Self::unpack_crc(self.message())?;

//        Self::unpack_header(self.message())?;

        self.unpack_payload()?;

        Ok(())
    }

    fn unpack_payload(&mut self) -> Result<(),Error>;
}

pub mod msg66;
pub mod msg84;
pub mod msg202;
pub mod msg203;
pub mod msg246;

// EOF
