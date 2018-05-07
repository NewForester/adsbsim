//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
// ------------------------------------------------------------------------- //
// MQTT Boiler Plate ------------------------------------------------------- //
// ------------------------------------------------------------------------- //
//
// This file contains boiler plate code to connect to an MQTT web socket server
// and subscribe to (possibly many) topics.  The application does not publish.
//
// The code is derived from examples seen on the Internet.  The aim was to
// implement the necessary and sufficient for the application.
//
// The callback that handles messages received is implemented in mavlink.js.
//

// mqttConnect() is called to establish a connection with an MQTT server
function mqttConnect() {
    client = new Paho.MQTT.Client(servername, Number(serverport), "/mqtt", clientid);

    client.onConnectionLost = onConnectionLost;
    client.onMessageArrived = onMessageArrived;

    client.connect({
        onSuccess: onConnect
    });
}

// onConnectionLost() is called if the connection with the server is severed
function onConnectionLost(responseObject) {
    if (responseObject.errorCode !== 0) {
        console.log("onConnectionLost:" + responseObject.errorMessage);
    }
}

// onConnect() is called when the application is now connected with the server
function onConnect() {
    console.log("connected to ws://" + servername + ":" + serverport + "//");

    var topics = subtopics.split(";");

    for (var ii in topics) {
        var topic = topics[ii];

        client.subscribe(topic, {
            invocationContext: {
                topic: topic,
            },
            onSuccess: onSuccessCallback,
            onFailure: onFailureCallback,
            timeout: 10
        });
    }
}

// onSuccessCallback() is called when subscription succeeds
function onSuccessCallback(context) {
    console.log("subscribed to topic:", context.invocationContext.topic );
    client.onMessageArrived = onMessageArrived;
}

// onFailureCallback() is called when a subscription fails
function onFailureCallback(context) {
    console.log("subscription failed for topic:", context.invocationContext.topic );
}

// EOF
