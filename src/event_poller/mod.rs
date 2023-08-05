use serde::Deserialize;
use reqwest::blocking::get;
use std::{error::Error};

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

    pub fn update_events(evs: &mut Vec<TMEvent>) -> Result<(), Box<dyn Error>> {
        for ev in evs.iter_mut() {
            // Iterating over all the events
            match poll_event(&ev.id) {
                // checking if was succes
                Ok(n) => {
                    println!("Succesfully polled {}: there are {} offers", ev.id, n);
                    ev.num_offers = n as u32; // Updating EV; note that this will change the top variable struct as we have borrowed the variable
                }
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    // Function poll_event
    //
    // Polls an event, and returns the nummer of offers. note that this function (for now) it blocking the thread
    //
    // Arguments
    //
    // * id - an &String that holds the event number that needs to be bolled
    //
    // Returns the number of offers as usize integer if no errors and an Box<error> in case there is an (underlying error)

    fn poll_event(id: &String) -> Result<usize, Box<dyn Error>> {
        let request_url = format!("{}{}", BASE_URL, id);
        let resp: TMResponse = get(request_url)?.json()?; // Doing the update, note the ? as this will propogate errors up!
        Ok(resp.offers.len()) // return the number of offers (we could have passed the event too)
    }
