//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
// ------------------------------------------------------------------------- //
// MAVLink Message Decode -------------------------------------------------- //
// ------------------------------------------------------------------------- //
//
// The code in this file handles the MAVLink messages received from one or more
// simulated ADS-B devices:
//
//   - 202 - ownship messages that gives the position etc. of 'this' uav
//   - 246 - traffic messages that give the position etc. of other uavs/ufos
//
// There is a single routine that acts as the MQTT callback called on receipt
// of each message.  The routine extracts pertinent data from the message and
// passes the date down to uavUpdate() so the position etc on the OSM map of
// uav/ufo objects can be updated.
//
// If you are familiar with JavaScript ArrayBuffers and DataViews then you
// should find the code pretty simple.
//
// ------------------------------------------------------------------------- //

// onMessageArrived() is called on receipt of a MAVLink message via MQTT
function onMessageArrived(message) {

    if (message.payloadBytes[5] == 202) {
        var icao = message.destinationName.split("/")[1];

        var payload = new DataView(message.payloadBytes.slice(6).buffer);

        var latitude = payload.getInt32(4, true) / 1.e7;
        var longitude = payload.getInt32(8, true) / 1.e7;
        var altitude = payload.getInt32(12, true) / 1.e3;

        uavUpdate(icao, 202, latitude, longitude, altitude);
    }

    if (message.payloadBytes[5] == 246) {
        var payload = new DataView(message.payloadBytes.slice(6).buffer);

        var icao = payload.getUint32(0, true).toString(16);

        var latitude = payload.getInt32(4, true) / 1.e7;
        var longitude = payload.getInt32(8, true) / 1.e7;
        var altitude = payload.getInt32(12, true) / 1.e3;

        uavUpdate(icao, 246, latitude, longitude, altitude);
    }
}

// EOF
