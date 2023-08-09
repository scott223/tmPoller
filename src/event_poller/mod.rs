use chrono::Local;
use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;

use crate::schema::{TMEvent, App};

const BASE_URL: &str = "https://availability.ticketmaster.eu/api/v2/TM_NL/resale/";

// TicketMaster API: full response (JSON deserialized)
#[derive(Deserialize, Debug)]
pub struct TMResponse {
    offers: Vec<Offer>,
}

// TicketMaster API: offer response (JSON deserialized)
#[derive(Deserialize, Debug)]
pub struct Offer {
    _id: String,
}

// Function update_events
//
// Updates the events
//
// Arguments
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)
pub fn update_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    for ev in app.events.iter_mut() {
        // Iterating over all the events
        match poll_event(ev) {
            // checking if was succes
            Ok(r_ev) => {

            }
            Err(err) => {
                // let error_line = format!("Error polling event {}: {}", ev.id, err);
                // app.submit_message(error_line.clone.().as_str());
            }
        }
    }
    Ok(())
}

// Function poll_event
//
// Polls an event, and returns the nummer of offers. note that this function (for now) it blocking the thread
//
// Arguments
// * id - an &String that holds the event number that needs to be bolled
//
// Returns the number of offers as usize integer if no errors and an Box<error> in case there is an (underlying error)
fn poll_event(ev: &mut TMEvent) -> Result<(), Box<dyn Error>> {
    let request_url = format!("{}{}", BASE_URL, ev.id);
    let response = get(request_url)?;

    ev.last_updated = Local::now();

    if response.status() == reqwest::StatusCode::OK {
        ev.last_update_status_code = response.status();
    } else {
        ev.last_update_status_code = response.status();
        return Err(response.status().as_str().into());
    }

    let result: TMResponse = response.json()?;
    ev.num_offers = result.offers.len();

    Ok(()) // return OK - no need to return the event as we have borrowed
}