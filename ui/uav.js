//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
// ------------------------------------------------------------------------- //
// Display of UAVs/UFOs on the OSM map ------------------------------------- //
// ------------------------------------------------------------------------- //
//
// The code in this file displays uavs/ufos on the OSM map.
//
// It implements a class (aerialVehicle)
//
//   - vehicle = new aerialVehicle(...) will display a new uav/ufo
//   - vehicle.update(...) will update the display of an existing uav/ufo
//
// There are two kinds of vehicles:
//
//   - uavs - displayed in blue that are assumed to be ADS-B transponders
//   - ufos - displayed in green ttat are assumed to tbe ADS-B transmitters
//
// Objects of type aerialVehicle are stored in the array vehicles so that the
// code will cope with any number of uavs/ufos.
//
// Note an earlier version of this code tried to use a standard JS map object to
// stored vehicles by icao address but this would not work.  It appeared a though
// the use of the Google Map API hide the standard object.  It is, of course,
// entirely possible that this is not the case and that the author is a total
// fuckwit.
//
// Note another earlier version attempted to use a plain JavaScript object as a
// map.  This just seemed to re-enforce our concerns about the author.
//
// ------------------------------------------------------------------------- //

// The aerialVehicle class for the display on the map of uav/ufo positions
function aerialVehicle (icao, position, colour, icon, radius) {
    this.icao = icao;
    this.polyline = [];

    this.marker = createMarker(position, icon);
    this.exclusionZone = createExclusionZone(position, colour, radius);
    this.regionOfInterest = createRegionOfInterest(position, colour);
    this.flightPath = createFlightPath(this.polyline, colour);

    this.updatePosition = function(position) {
        this.marker.setPosition(position);
        this.exclusionZone.setCenter(position);
        this.regionOfInterest.setCenter(position);

        this.polyline.push(position);

        this.flightPath.setPath(this.polyline);
    }
}

// The collection of aerialVehicle objects
var vehicles = [];

// uavUpdate() puts a ufo/uav on the map or updates its position
function uavUpdate(icao, messageId, latitude, longitude, altitude)
{
    var position = new google.maps.LatLng(latitude, longitude);

    for (var ii in vehicles) {
        if (icao == vehicles[ii].icao) {
            vehicles[ii].updatePosition(position);
            return;
        }
    }

    if (messageId == 202)
        vehicles.push(new aerialVehicle(icao, position, "#0000FF", "quadcopter.png", 10));
    else
        vehicles.push(new aerialVehicle(icao, position, "#00FF00", "ufo_smaller.png", 20));
}

// EOF
