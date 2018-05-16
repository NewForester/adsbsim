//! ADS-B Simulator - WIP
//
// © NewForester, 2018.  Available under MIT licence terms.
//
extern crate byteorder;
extern crate crc16;

pub mod msg66;
pub mod msg84;
pub mod msg202;
pub mod msg203;
pub mod msg246;

use std::io::Error;

use self::byteorder::{LittleEndian, WriteBytesExt};

static mut SEQNO_OUT: u8 = 0;

const SHUTUP: usize = 256;

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

    fn serialise(&mut self) -> &mut Self;

    fn dump(&self) -> &Self;

    fn message(&self) -> &[u8];

    fn dump_message(message: &[u8]) -> () {
        print!("Dump message {:3}:", Self::MSGID);
        for ii in message {
            print!(" {:02x}", ii);
        }
        println!();
    }

    fn serialise_message(&self) -> Vec<u8> {

        let mut buffy: Vec<u8> = Vec::with_capacity(6 + 256 + 2);

        match Self::pack_message(self, &mut buffy) {
            Ok(_)  => {
                ();
            }
            Err(_) => {
                println!("Serialisation of message {} failed", Self::MSGID);
                buffy.clear();
            }
        }

        buffy
    }

    fn deserialise_message(&mut self, buffy: &Vec<u8>) -> &mut Self {
        let mut known_size: [u8; SHUTUP] = [0; SHUTUP];

        for ii in 0 .. Self::PAYLEN {
            known_size[ii] = buffy[6 + ii];
        }

        match self.unpack_payload(&known_size[ .. ]) {
            Ok(_)  => {
                ();
            }
            Err(_) => {
                println!("Deserialisation of message {} failed", Self::MSGID);
            }
        }

        self
    }

    fn pack_message(&self, buffy: &mut Vec<u8>) -> Result<(),Error> {
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

        buffy.push(header.mavstx);
        buffy.push(header.paylen);
        buffy.push(header.seqno);
        buffy.push(header.sysid);
        buffy.push(header.compid);
        buffy.push(header.msgid);

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

    fn unpack_payload(&mut self, payload: &[u8]) -> Result<(),Error>;
}

// EOF
