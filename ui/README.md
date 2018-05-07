<!-- adsbsim-ui by NewForester:  a program to simulate ADS-B input as MAVLink messages -->

# ADS-B Simulator - UI

The _adsbsim_ (ADS-B Simulator) does not have a UI by design.
It does not need one:  it should be sufficient to capture and analyse the numbers.

Nonetheless, a visual crutch is nice for some if only to avoid loss of orientation followed by loss of lunch.

The _adsbsim_ uses MQTT by design.
If the _LRE wrapper_ subscribes to MQTT topics that tell it where it is and where the obstacles it must avoid are,
then so can a UI program and it can do its thing quietly without demanding to be centre stage.

Choosing to construct a browser application means using a more or less ubiquitous run-time:
this UI is implemented as a web-page using JavaScript.

## UAVs and UFOs

Two TLAs.  UAV stands for Unmanned Aerial Vehicle, UFO for Unidentified Flying Obstacle.

An ADS-B device (such as is assumed by the simulator) gives out position and other information about the UAV on which it is mounted and
(possibly many) UFOs that it detects.

UFOs must be at least an ADS-B transmitter and a UAV at least an ADS-B receiver.

The ADS-B Simulator goes a little bit further:

  * a UAV is taken to be 'active' (it can a should take action to avoid a collision) so should be an ADS-B transponder
  * a UFO is taken to be 'passive' (it cannot take action to avoid a collision) so it is only an ADS-B transmitter

This program treats UAV and UFO the same way (it is only a display program) except that
it uses different icons for UAV and UFO and different colours: UAVs are blue, while UFOs are green.

## Screamfuls

There is one page comprising two screens.

The first screen is a form that allows the tester to set the MQTT parameters.
Doing this repeatedly may become tedious.
When is does, it is suggested you change the defaults in _ui.js_ (the first stuff that isn't comments).

The _UAV/UFO Topics_ parameter deserves note and is discussed in further detail below.

The second screen is reached by clicking on the "Accept Parameters and Show simulation" button.

This displays a map, by default, in satellite view.  You can select alternatives or change the default (in _osm.js_).

The map is centred over a well known secret location.  You can change the default by changing _secretLocation_ (in _osm.js_).

If you cannot see any UAVs or UFOs that may be because:

  * the simulator is not running (so start it)
  * the craft have left the field of view (try scrolling out)
  * the _UAV/UFO Topics_ parameter is invalid (see below).

## UAV/UFO Topics

The ADS-B Simulator uses the following simple convention for ADS-B topics: `/icaoAddress/messageId`.
Where:

  * _icaoAddress_ is the (simulated) ICAO address assigned to a UAV - these are unique;
  * _messageId_ is a MAVLink message id - messages with this id are published to this subtopic;

There is no pretence that this is the best convention or that it would have been adequate for all the tests that had been foreseen.

### Example 1

Suppose the simulator instance is generating ADS-B data for a UAV assigned the ICAO address `0x300159`.

A _UAV/UFO Topics_ parameter of `/300159/#` will subscribe to all messages published by that simulator.
It will receive MAVLink 202 and 246 messages.

The UI will use the 202 messages to display the progress of the UAV in blue and
the 246 messages to display the progress of UFOs in green.
If there are 246 messages for more than one UFO with different ICAO addresses,
the program will display the progress of each UFO.

### Example 2

Suppose two simulator instances are generating ADS-B data for two UAVs assigned the ICAO addresses
`0x300159` and `0x151060`.

Each simulator (if set up correctly) will be see the other as a UFO.
We want the UI to show the UAVs as a UAVs and not as UFOs so we should use a _UAV/UFO Topics_ parameter of
`/300159/202;/151060/202`.

The UI program would subscribe to two topics and only receive the 202 messages of each UAV.

You may subscribe to as many topics as you need - just use the semi-colon between each -
and can narrow down the topic to get the messages you need -
you could use `/300159/246;/151060/246` to see the UAVs as UFOs but that would be silly.

## Icon Caveat

The program uses two icons:  one to represent UAVs and one to represent UFOs but for copyright reasons the program is distributed without icons.

This should not affect program operation but if you like to have icons, go find a couple and alter the code (in _uav.js_).
Replace the names "quadcopter.png" and "ufo_smaller.png" with those of your chosen icons and leave the icons in the _ui_ directory beside the README.md
and the JavaScript code.

---

*adsbsim* by NewForester.
Available under MIT licence terms.

<!-- EOF -->
