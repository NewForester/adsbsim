//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
// ------------------------------------------------------------------------- //
// UI Startup and Parameter Form ------------------------------------------- //
// ------------------------------------------------------------------------- //
//
// On start up, the UI presents a simple form that allows the user to override
// the default values for parameters.  The parameters are MQTT connection and
// subscription parameters.
//

var servername = "127.0.0.1";
var serverport = "9001";
var clientid = "AdsbSimViewer";
var subtopics = "/300159/#";
//var subtopics = "/300159/202;/151060/202";

// setParameters() copies the application's parameters to the DOM for display
function setParameters() {
    document.getElementById("mqttParameters").servername.value = servername;
    document.getElementById("mqttParameters").serverport.value = serverport;
    document.getElementById("mqttParameters").clientid.value = clientid;
    document.getElementById("mqttParameters").subtopics.value = subtopics;
}

// setParameters() copies the application's parameters from the DOM for use
function getParameters() {
    servername = document.getElementById("mqttParameters").servername.value;
    serverport = document.getElementById("mqttParameters").serverport.value;
    clientid = document.getElementById("mqttParameters").clientid.value;
    subtopics = document.getElementById("mqttParameters").subtopics.value;
}

// called when the web page has been loaded - initiates the OSM connection
document.onreadystatechange = function () {
    if (document.readyState == "complete") {
        document.getElementById("map").style.display = 'none';

        setParameters();

        osmConnect();
    }
}

// onShowSimulation() shows the OSM map and initiates the MQTT connection
function onShowSimulation() {
    getParameters();

    document.getElementById("welcome").style.display = 'none';
    document.getElementById("map").style.display = 'block';

    mqttConnect();
}

// EOF
