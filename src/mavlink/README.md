<!-- adsbsim-mavlink by NewForester:  a program to simulate ADS-B input as MAVLink messages -->

# ADS-B Simulator

This is a simple program implemented in Rust.

The program was designed in the Unix tradition:  it does one thing with an aim to doing it well.

What it will do is mimic an ADS-B device:  for everything else there is the Internet.

This README.md describes the MAVLink message module which comprises traits and several implementations.

## Module Structure

Rust encourages modular program structure in the finest time honoured tradition and
spoils it all by mandating the name of the module level compilation unit.

The _mavlink_ module has half a dozen modules:

  * mod.rs
  * msg202.rs
  * msg203.rs
  * msg246.rs
  * msg66.rs
  * msg84.rs

_mod.rs_ is the mandated name of the module file.
The others are implementations for MAVLink messages 202, 203, 246, 66 and 84.

### The mavlink Trait Definitions - mod.rs

The _mavlink_ module defines two traits:  _serialise_ and _deserialise_ that all MAVLink messages should implement.
In practice, the module provides the common parts of these traits so the individual message implementation need only
provide the implementations for the routines to pack and unpack the message specific payload.
These routines are straight forward.

The trait implementation involves some duplication because Rust is a safe.  See below.

### The mavlink Message Implementations

Historically, the _serialise_ trait was implemented first at a time when the ADS-B Simulator program's remit was limited to ADS-B message generation.
The _deserialise_ trait was implementation later when receipt of MAVLink 84 and 202 messages was added.
The new trait was not implemented for the other messages but would it would easy enough to back fill as required.

Use the existing MAVLink 202 messages as a 'template' for more message implementations.  Each implementation comprises:

  * the definition of a structure to represent the message;
  * a implementation section to provide methods for the message structure;
  * an implementation section for the _mavlink_ message trait.

The implementation section for the messages structure has to provide a `new()` method.
Other methods are optional however, getter and setter methods to convert from MAVLink field representations to something useful in Rust are recommended.

A message implementation module may be tediously long but each is simple to write.

## Unavoidable Duplication

MAVLink is a maverick network 'protocol':  it does not use network byte order.
This is, in principle, to minimise protocol overhead on machines likely to implement the protocol.
It results in two field orders:  the documented one and the one used in the real world.

Rust explicitly does not guarantee the field order of structures and so this MAVLink distinction is not useful.
Eac  message much be packed up in the correct order before transmission and unpacked on receipt.
This is the reason for the MAVLink message trait.
The _byteorder_ crate is used for this.

However, Rust is 'safe' and insists on determining whether these pack and unpack routines are 'safe':
it will not allow buffer overflow arising from packing too much into two small a buffer or
from attempting to unpack too much data from a buffer that is too small.
So fixed sized buffers much be used.

However, MAVLink messages are of different sizes that are known and fixed but not until run time.
This gives rise to the need to copy MAVLink message contents to and from buffers whose size is known and fixed at compile time
and to every message structure instance containing the message data in packed and unpacked forms.
Ho hum.

---

*adsbsim* by NewForester.
Available under MIT licence terms.

<!-- EOF -->
