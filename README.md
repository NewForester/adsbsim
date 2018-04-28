<!-- adsbsim by NewForester:  a program to simulate ADS-B input as MAVLink messages -->

# ADSB-B Simulator

This is a simple program implemented in Rust.

The program was designed in the Unix tradition:  it does one thing with an aim to doing it well.

What it will do is mimic an ADS-B device:  for everything else there is the Internet.

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
the simulator will only send messages as UDP packets.

The simulator generates MAVLink messages with a valid header and CRC.
The message sequence number is incremented for each message sent.

There is no guarantee that all payload fields contain valid data:
but those necessary for predicting collisions between the UAV and a UFO will, by definition, contain valid data.

The simulator will simulate the UAV and one UFO each either flying at constant velocity or stationary.
Each simulation is repeatable.
There is no need to risk real craft and real devices or wait for suitable weather conditions.

## Simulator Parameters

The simulator is a simple command line program that takes the following parameters:

  * -i= // UDP network parameters
  * -uav= // UAV parameters
  * -ufo= // UFO parameters

### UAV and UFO Parameters

UAV and UFO parameters are the same.
Their purpose is to specify the starting position of the vehicle / obstacle (craft) and its velocity:
the are sufficient to define straight line flight paths.

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
However, this is Rust and the delimiters are still required.

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
this is Rust and Rust is _safe_ and getting it to accept sloppy, human input, is hard.

Note all parameters are taken to be floating point so feel free to specify parameters down to the nth decimal point.

### Network Parameters

The network parameters have the from:

```
    -i=src:dst:host
```

The _host_ may be omitted, in which case it defaults to 127.0.0.1.

The _dst_ and _host_ parameters specify the INET socket address to which the simulator sends messages.

The _src_ specifies a socket port the simulator sends messages from.
It binds to the port to 'reserve' it:  it does not receive messages.

## What the Rust ?

How to invoke / compile / build Rust programs is, for the time being, beyond the scope of these notes.

---

*adsbsim* by NewForester.
Available under MIT licence terms.

<!-- EOF -->
