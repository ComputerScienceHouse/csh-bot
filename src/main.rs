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

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate actix_web;

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
}

#[derive(Debug, Deserialize)]
struct Request {
    queryResult: QueryResult,
}

#[derive(Debug, Serialize)]
struct Response {
    fulfillmentText: String,
}

fn fulfillment(info: Json<Request>) -> Result<Json<Response>> {
    println!("Received a fulfillment request: {:#?}", info);

    let response: Option<String> = match &*info.queryResult.intent.name {
        "projects/shitpost-5f519/agent/intents/db2d53af-2d4e-47a5-a6e8-e27adb491ab7" => Some(turn_lights_on()),
        "projects/shitpost-5f519/agent/intents/5b3ee694-7626-493c-a5e4-f403b19444b5" => Some(turn_lights_off()),
        _ => None
    };

    let response = match response {
        Some(text) => Response { fulfillmentText: text },
        None => Response { fulfillmentText: format!("Sorry, I don't know how to do that yet.") },
    };

    Ok(Json(response))
}

fn turn_lights_on() -> String {
    format!("The lights are on _for reals_")
}

fn turn_lights_off() -> String {
    format!("The lights are off _for reals_")
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.method(http::Method::POST).with(fulfillment))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}