//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
// ------------------------------------------------------------------------- //
// OSM Boiler Plate -------------------------------------------------------- //
// ------------------------------------------------------------------------- //
//
// This file contains boiler plate code to create an OSM map and uav/ufo
// markers etc for display on the map.
//
// Each uav/ufo is displayed as:
//
//    - an icon marker (that appears over the actual position)
//    - an exclusion zone representing (a disappointingly small circle)
//    - an area of interest (in which any other uav/ufo is a potential concern)
//    - a flight path (which is historical)
//
// The code is derived from examples seen on the Internet.  The aim was to
// implement common code to support the display of many uav/ufo objects.
//
// The code uses OSM and the Google Maps API but not Google Maps proper as that
// requires the possession of a Google Map capable mobile 'phone.
//
// However, evidence is that the 'satellite' view used by default is from
// Google Maps and does not require an API key.
//
// ------------------------------------------------------------------------- //

var osmMap;

// createMarker() puts a uav/ufo icon on the map
function createMarker(position, icon) {
    var marker = new google.maps.Marker({
        position:       position,
        icon:           icon,
    });
    marker.setMap((osmMap));
    return marker;
}

// createExclusionZone() draws an exclusion zone around a uav/ufo on the map
function createExclusionZone(position, colour, radius) {
    var exclusionZone = new google.maps.Circle({
        center:         position,
        radius:         radius,
        strokeColor:    colour,
        strokeOpacity:  1.0,
        strokeWeight:   1,
        fillColor:      colour,
        fillOpacity:    0.5,
    });
    exclusionZone.setMap(osmMap);
    return exclusionZone;
}

// createRegionOfInterest() draws the region of interest for a uav/ufo on the map
function createRegionOfInterest(position, colour) {
    var regionOfInterest = new google.maps.Circle({
        center:         position,
        radius:         2000,
        strokeColor:    colour,
        strokeOpacity:  1.0,
        strokeWeight:   2,
//        fillColor:      colour,
        fillOpacity:    0.0,
    });
    regionOfInterest.setMap(osmMap);
    return regionOfInterest;
}

// createFlightPath() draws the historic flight path of a uav/ufo on the map
function createFlightPath(polyline, colour) {
    var flightPath = new google.maps.Polyline({
        path:           polyline,
        strokeColor:    colour,
        strokeOpacity:  0.8,
        strokeWeight:   2
    });
    flightPath.setMap(osmMap);
    return flightPath;
}

// osmConnect() creates and displays an OSM map
function osmConnect()
{
    var secretLocation = new google.maps.LatLng(51.1011677, -2.0513459);

    var mapTypeIds = [];
    for(var type in google.maps.MapTypeId) {
        mapTypeIds.push(google.maps.MapTypeId[type]);
    }
    mapTypeIds.push("OSM");

    osmMap = new google.maps.Map(document.getElementById("map"), {
        center: secretLocation,
        zoom: 15,
        mapTypeId: "satellite",
        mapTypeControlOptions: {
            mapTypeIds: mapTypeIds
        }
    });

    osmMap.mapTypes.set("OSM", new google.maps.ImageMapType({
        getTileUrl: function(coord, zoom) {
            return "http://tile.openstreetmap.org/" + zoom + "/" + coord.x + "/" + coord.y + ".png";
        },
        tileSize: new google.maps.Size(256, 256),
        name: "OpenStreetMap",
        maxZoom: 18
    }));
}

// EOF
