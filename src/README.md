<!-- adsbsim-src by NewForester:  a program to simulate ADS-B input as MAVLink messages -->

# ADS-B Simulator

This is a simple program implemented in Rust.

The program was designed in the Unix tradition:  it does one thing with an aim to doing it well.

What it will do is mimic an ADS-B device:  for everything else there is the Internet.

This README.md describes the program's modular structure and provides other 'program global' design notes.

## Module Structure

Rust encourages modular program structure in the finest time honoured tradition and
spoils it all by mandating the name of the program level compilation unit.

The simulator has four modules:

  * coords.rs - a representation of a UAV/UFO's position and velocity
  * main.rs - the rambler
  * mqtt.rs - the MQTT client
  * mavlink.rs - an abstraction of a MAVLink message with several implementations.

### main.rs

The _main_ module should:

  * process command line parameters,
  * initialise external interfaces,
  * establish inter-thread communications
  * spawn worker threads.

This it does but in a rather messy fashion.

The main routine creates rather a lot of data items that are 'global' in the sense that they permeate the
entire application but Rust hates 'global data' because it is a control freak.

As a result the main routine isn't and the bulk of this module's code is in the `producer()` function that
rambles on for almost an obscene 150 lines of code.
It embodies the main logic of the program.
It runs in its own separate execution context.

The `producer()` function is responsible for generating a burst of MAVLink messages once a second just as the ADS-D device would.
In simple scenarios, the burst of messages represent the progress of a UAV and a UFO on straight line trajectories.

In more sophisticated scenarios that simulate collision avoidance, incoming MAVLink 84 messages are used to modify the course of the UAV
and incoming MAVLink 202 messages to generate the course of the UFO.

### mqtt.rs

The _mqtt_ module is an application specific wrapper around the _mosquitto_client_ crate.
This client was picked because it appeared to be the nearest thing the cargo repository had to an 'official' library.

The _mosquitto_client_ crate is simply a wrapper around the C library for _mosquitto_ clients and
_mosquitto_ is the 'other' Eclipse/Apache MQTT project - the first being the _paho_ project.

The `publish()` and `subscribe()` routines are the only not quite trivial functions in here as they understand
the simulator's use of MQTT topics.

### coords.rs

The simulator needs the 3D co-ordinates and 3D velocities of UFOs and UAVs.
The _coords_ module provides a structure to hold these; functions to manipulate them and accessors functions to set/get individual values.

For the _main_ module it provides a type to represent UAVs and UFOs and for the _mavlink_ module it allows fiddly code to be expressed tidily.

### mavlink

The _mavlink_ module is a subdirectory with a number of sub modules of its own.
See [README.md](./mavlink/README.md) in the _mavlink_ directory.

At the program level, it provides:

  * the means to create MAVLink message;
  * pack/unpack MAVLink messages (before transmission/after receipt);
  * get/set the fields in MAVLink messages

## Many UFOs

The ADS-B simulator is currently capable of simulating a single UFO;  the intention was to continue development until it could simulate many.

It is believed that this should not be too hard (Rust not withstanding) and that all the necessary changes begin, and could end, in the _main_ module.
It might be appropriate to push code down to other modules but there is nothing in these that would definitely have to change for multi-UFO operation.

The UFO at present is represented by:

```
    let mut ufo = CwithV::new();
```

This would have to become a container.
Individual UFOs are distinguished using an ICAO address,
so perhaps an 'associative array' would be more appropriate than a simple 'vector'.

Places where `ufo` is used would need a looping constructs of some sort or other.
The `for` loop that generates a message burst might be replaced by two.

It should still be possible to use just one `traffic report` message instance but the ICAO address would have to be set for each message transmission.

It should be possible to use any combination of 'CLI parameter' UFOs and 'received 202 message' UFOs.
The snag here is that the `-ufo=` parameter does not at present include an ICAO address.
Consider that a design change that may not be backwards compatible.

The `ufoinitialised` flag (a last minute addition) is the final alteration at which point refactoring might well push this flag down into the `CwithV` structure.

---

*adsbsim* by NewForester.
Available under MIT licence terms.

<!-- EOF -->
