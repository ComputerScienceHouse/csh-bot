//! The fulfillment service for a DialogFlow agent built to control CSH projects.
//!
//! DialogFlow is a service (now run by Google) that performs natural-language processing on
//! English string inputs, performs actions based on the _intent_ of the user, and returns an
//! English output that describes the actions taken or the status requested. To achieve the middle
//! step - performing actions - DialogFlow will send an HTTP request to an address of our choosing.
//! At the address we provide, we must host an API server that consumes the request as JSON,
//! then performs actions however we like. We then return an HTTP response that contains the
//! string that the agent will return to the user.
//!
//! The documentation on how to parse requests and how to construct responses is given
//! on the [DialogFlow docs] site. This project uses [actix-web] to implement the API server,
//! and makes use of [serde] to deserialize the requests and to serialize the responses.
//!
//! [DialogFlow docs]: https://dialogflow.com/docs/fulfillment/how-it-works
//! [actix-web]: https://github.com/actix/actix-web
//! [serde]: https://github.com/serde-rs/serde
//!
//! # Development Notes
//!
//! Since DialogFlow is a service that is hosted online, we need some way to expose our
//! development server's port publicly. There's a tool called [ngrok] that will generate a
//! publicly-facing URL, and forward all traffic received there to a specified port on your
//! machine locally. Download ngrok, and run `./ngrok http 8000` to forward traffic to port 8000.
//! Then use the generated `https` address as the fulfillment endpoint for DialogFlow.
//!
//! [ngrok]: https://ngrok.com/

#![allow(non_snake_case)]
#![deny(warnings)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate actix_web;

use std::collections::HashMap;
use actix_web::{server, http, App, Json, Result};

#[derive(Debug, Deserialize)]
struct Intent {
    name: String,
    displayName: String,
}

#[derive(Debug, Deserialize)]
struct QueryResult {
    queryText: String,
    intent: Intent,
    parameters: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Request {
    queryResult: QueryResult,
}

#[derive(Debug, Serialize)]
struct Response {
    fulfillmentText: String,
}

/// Receive the request, execute an intent handler, and return a response.
///
/// This is basically a router that switches based on the intent ID of the
/// request received from DialogFlow. Based on the intent, we extract the
/// entities (sentence parameters) from the request and pass execution to an
/// intent handler, passing the entity data to the handler.
fn fulfillment(info: Json<Request>) -> Result<Json<Response>> {
    println!("Received a fulfillment request: {:#?}", info);

    let response: Option<String> = match &*info.queryResult.intent.name {
        "projects/computersentiencehouse/agent/intents/a3fbc0a8-8323-4d0b-8085-eb057c6f6129" => {
            // Extract "queryResult.parameters.Room" from JSON request
            let room = info.queryResult.parameters.get("Room")
                .expect("should have a 'Room' entitiy");
            let enabled = info.queryResult.parameters.get("Enabled")
                .expect("should have an 'Enabled' entity");
            Some(set_lights(room, enabled))
        },
        _ => None
    };

    let response = match response {
        Some(text) => Response { fulfillmentText: text },
        None => Response { fulfillmentText: format!("Sorry, I don't know how to do that yet.") },
    };

    Ok(Json(response))
}

/// Intent handler for the "Set Lights" intent.
///
/// # Entities
///
/// * `Room`: Represents which room's lights to turn on or off. The possible
/// values are `Lounge`, `Library`, and `User Center`.
///
/// * `Enabled`: Whether to turn the lights on or off. The possible values
/// are `On` or `Off`.
fn set_lights(room: &str, enabled: &str) -> String {
    format!("I'm turning the lights in the {} {}!",
            room.to_lowercase(),
            enabled.to_lowercase(),
    )
}

fn main() {
    // Open an HTTP server on port 8000, using the "fulfillment" function
    // to handle all POST requests sent to the index ("/") route.
    server::new(|| {
        App::new()
            .resource("/", |r| r.method(http::Method::POST).with(fulfillment))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
