use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;
use chrono::{Local};

use crate::schema::TMEvent;

const BASE_URL: &str = "https://availability.ticketmaster.eu/api/v2/TM_NL/resale/";

#[derive(Deserialize, Debug)]
pub struct TMResponse {
    offers: Vec<Offer>,
}

// TicketMaster API: offer response (JSON deserialized)
#[derive(Deserialize, Debug)]
pub struct Offer {
    id: String,
}

// Function update_events
//
// Updates the events
//
// Arguments
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)

pub fn update_events(evs: &mut Vec<TMEvent>) -> Result<(), Box<dyn Error>> {
    
    for ev in evs.iter_mut() {
        // Iterating over all the events
        match poll_event(ev) {
            // checking if was succes
            Ok(r_ev) => {
                println!("Succesfully polled {}: there are {} offers", ev.id, ev.num_offers);
           }
            Err(err) => { 
                eprintln!("Error polling event {}: {}",ev.id,err)
            },
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
    if response.status() == reqwest::StatusCode::OK {
        ev.last_update_status_code = response.status();    
    } else {
        ev.last_update_status_code = response.status();
        return Err(response.status().as_str().into());
    }
    
    let result: TMResponse = response.json()?;
    (ev.num_offers, ev.last_updated) = (result.offers.len(), Local::now());
    Ok(())// return OK - no need to return the event as we have borrowed
}
