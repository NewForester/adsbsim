<!-- adsbsim by NewForester:  a program to simulate ADS-B input as MAVLink messages -->

# ADS-B Simulator

This is a simple program implemented in Rust.
A UI implemented in JavaScript as included as an appendix.

The program was designed in the Unix tradition:  it does one thing with an aim to doing it well.

What it will do is mimic an ADS-B device:  for everything else there is the Internet.

## Keywords

ADS-B, MAVLink, MQTT, Rust.

If your keyword search should include at least three of these keywords.
If it does not then it is likely that this simulator will be of no practical value to you.

## ADS-B Operation

The simulator (and device being simulated) will generate a burst of valid MAVLink messages once a second:

  * message 66 (data stream request)
  * message 203 (status)
  * message 202 (ownship)
  * message 246 (traffic report)

The 202 message contains information about the position (and so forth) of an unmanned air vehicle (UAV)
to which the device is attached.
The UAV is presumed to be in flight.
The 246 message contains information about the position (and so forth) of some other air vehicle (or obstacle) fitted
with an ADS-B transmitter/transponder (UFO).

A real device would generate a 246 message for each UFO:  the present version of the simulator will generate a message for
only one UFO.

A real device might be expected to send MAVLink messages over a serial link or as UDP packets over a network:
the simulator will send messages as UDP packets or publish them to an MQTT broker.

The simulator generates MAVLink messages with a valid header and CRC.
The message sequence number is incremented for each message sent.

There is no guarantee that all payload fields contain valid data:
but those necessary for predicting collisions between the UAV and a UFO will, of necessity, contain valid data.

The simulator will simulate the UAV and one UFO each either flying at constant velocity or stationary.
Each simulation is repeatable.
There is no need to risk real craft and real devices or wait for suitable weather conditions.

When an MQTT broker is used, the simulator can be set to subscribe to MAVLink 84 messages that indicate a change of course by the (simulated) UAV.
The simulator will modify the UAV position it reports in MAVLink 202 messages appropriately.

When an MQTT broker is used, the simulator can be set to subscribe to MAVLink 202 messages published by some other (simulated) UAV.
The simulator will use the position and velocity data in these 202 messages to generate its MAVLink 246 messages.
This allows the simulation of scenarios that involve two UAVs with collision detection and avoidance capability.

## Simulator Parameters

The simulator is a simple command line program that takes the following parameters:

  * -i= // UDP network parameters
  * -uav= // UAV parameters
  * -ufo= // UFO parameters
  * -mq= // MQTT parameters

The UDP network parameters have been largely superseded by the MQTT parameters.
Although the program should still support UDP operation, the small amount of code in `main.rs` that does so has not been used in some time.

### UAV and UFO Parameters

UAV and UFO parameters are the same.
Their purpose is to specify the starting position of the vehicle / obstacle (craft) and its velocity:
they are sufficient to define straight line flight paths.

The parameter values have the form:

```
  (x y z),(∆x ∆y ∆z)
```

The delimiters are required:  this is Rust and Rust is ~~pedantic~~ safe.

The parameters x, y and z are the latitude, longitude and altitude of the craft's starting position.
By convention, x is in degrees latitude, y in degrees longitude and z in m above mean sea level.

The parameters ∆x, ∆y and ∆z are the north/south velocity, east/west velocity and rate of climb of the craft.
All are expressed in m/s.

There are default co-ordinates and velocities: a secret location and stationary.

For ease of use, it is possible to specify the craft's starting position relative to the secret default location.
Displacements are expressed in m and the `m` is required.

Thus to specify:

  * the UAV starting from the secret location travelling east at 10 m/s
  * and a stationary UFO 100 m/s east of the secret location

one might specify:

```
  -uav="(  ),(0 10 0)" -ufo="(0m 100m ),(  )"
```

There, you do not need to know the secret location but you do need the spaces:
this is Rust and Rust is _safe_ and getting it to accept sloppy, human input, is ___hard___.

Note all parameters are taken to be floating point so feel free to specify parameters down to the nth decimal point.

### UDP Network Parameters

The UDP network parameters have the from:

```
    -i=src:dst:host
```

The _host_ may be omitted, in which case it defaults to 127.0.0.1.

The _dst_ and _host_ parameters specify the INET socket address to which the simulator sends messages.

The _src_ specifies a socket port the simulator sends messages from.
It binds to the port to 'reserve' it:  it does not receive messages on this port.

### MQTT Network Parameters

MQTT network parameters are passed to the program from the command line using the `-mq` flag,
which has the general form:

```
    -mq=clientId,brokerHost:brokerPort,pubTopic,subTopics
```

where:

 * clientId is required but arbitrary (_$$_ is sufficient);
 * brokerHost:brokerPort identify the MQTT broker (default _127.0.0.1:1883_);
 * pubTopic is the MQTT topic root to which the simulator publishes messages (see below);
 * subTopics is a `;` (semi-colon) separated list of MQTT topics to which the simulator is to subscribe to receive messages;

This parameter is optional but unavoidable in all but the simplest of test configurations.
When used, it overrides any `-i` parameter.

In principle, both the `-ufo` and this parameter may be used to specify a source of `ufo` traffic data but, at present,
the simulator supports only one `ufo` so it is an either/or situation.

The simulator has been used with the convention that messages with id _xx_ are published to _pubtopic/xx_ and _pubtopic_ is the ICAO address of the craft.
Subscriptions may then, in principle, be a one or more craft, all or just a subset of the messages associated with an individual craft and combinations thereof.

### Example 1

Suppose we wish to simulate a UAV approaching a stationary UFO.

The parameters:

```rust
    target/debug/adsbsim -uav="(  ),(0 10 0)" -ufo="(0m 100m ),(  )" -mq="Sim-$$,,/151060"
```

  * the initial positions of the UAV and UFO are the 'secret location' and 100 m east of there;
  * the UAV velocity is east at 10 m/s - the UFO is stationary;
  * 'ownship' messages will be published to _/151060/202_;
  * 'traffic report' messages will be published to _/151060/246_;

The traffic report message will report the ICAO address of the UFO as `0x300119`.
This is hard-coded (aka the program is unfinished).

### Example 2

Suppose we wish to simulate two UAVs approaching each other.

The 'traffic report' (246) messages must be generated not from a fixed path but using the 'ownship' (202) messages published by the other craft.
Two instances of the simulator must be run:

```rust
    target/debug/adsbsim -uav='(0m -750m 0m),(0 20 0)' -mq="Rust-$$,,/300159:/151060/202"
    target/debug/adsbsim -uav='(0m 750m 0m),(0 -20 0)' -mq="Rust-$$,,/151060:/300159/202"
```

  * the initial positions of the craft are 750m west and east of the 'secret location';
  * each approaches the other at 20 m/s;
  * the craft have ICAO addresses _0x301059_ and _0x151060_;
  * the craft publish their 'ownship' positions to _/300159/202_ and _/151060/202_;
  * the position of the other craft is obtained by subscribing to _/151060/202_ and _/300159/202_;

### Example 3

Suppose the software under test is introduced and is configured to publish 84 messages (collision avoidance change of course messages)
following the same topic naming conventions.

```rust
    target/debug/adsbsim -uav='(0m -750m 0m),(0 20 0)' -mq="Rust-$$,,/300159:/300159/84;/151060/202"
    target/debug/adsbsim -uav='(0m 750m 0m),(0 -20 0)' -mq="Rust-$$,,/151060:/151060/84;/300159/202"
```

Each craft is subscribed to receive the 84 messages for itself and is able to adjust its course in the 202 messages it generates appropriately.

Since everything is via MQTT, passive loggers, analysers and visualisers can be added unobtrusively.

## More Details

See [README.md](./src/README.md) in the _src_ sub-directory for more information on the ADS-B Simulator proper and
[README.md](./ui/README.md) in the _ui_ sub-directory for more information on the simple UI for the ADS-B Simulator.

## What the Rust ?

How to invoke / compile / build Rust programs is, for the time being, beyond the scope of these notes.

If you need to Google Rust then you probably should find yourself another simulator.

---

*adsbsim* by NewForester.
Available under MIT licence terms.

<!-- EOF -->
