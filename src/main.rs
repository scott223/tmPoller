use std::{error::Error, time::Duration};
use crossterm::event::{self, Event, KeyCode};
use reqwest::blocking::get;
use serde::Deserialize;

const BASE_URL: &str = "https://availability.ticketmaster.eu/api/v2/TM_NL/resale/";

#[derive(Debug)]
struct TMEvent {
    id: String,
    num_offers: u32,
}

#[derive(Deserialize, Debug)]
struct TMResponse {
    offers: Vec<Offer>,
}

#[derive(Deserialize, Debug)]
struct Offer {
    id: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variables
    let mut tm_events = vec![
        TMEvent {
            id: "284753".to_string(),
            offers: 0,
        },
        TMEvent {
            id: "280409".to_string(),
            offers: 0,
        },
    ];
    println!("Initialized, running program ...");

    // Running main loop
    'mainloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // User pressed ESC or 'q', breaking the main loop
                        break 'mainloop;
                    }
                    KeyCode::Char('p') => {
                        // User pressed 'p', forcing an update of the events
                        println!("User forced polling of all events");
                        match update_events(&mut tm_events) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(evnt) => println!("All events updated"),
                            Err(e) => println!("Error: {}", e),
                        }
                        println!("Data dump: {:?}", tm_events); // temp: data dump
                    }
                    _ => {}
                }
            }
        }
    }

    // Preparing for exit
    println!("Quiting, bye!");
    Ok(())
}

fn update_events(evs: &mut Vec<TMEvent>) -> Result<(), Box<dyn Error>> {
    for ev in evs.iter_mut() {
        // Iterating over all the events
        match poll_event(&ev.id) {
            // checking if was succes
            Ok(n) => {
                println!("Succesfully polled {}: there are {} offers", ev.id, n);
                ev.offers = n as u32; // Updating EV; note that this will change the top variable struct as we have borrowed the variable
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

fn poll_event(id: &String) -> Result<usize, Box<dyn Error>> {
    let request_url = format!("{}{}", BASE_URL, id);
    let resp: TMResponse = get(request_url)?.json()?; // Doing the update, note the ? as this will propogate errors up!
    Ok(resp.offers.len()) // return the number of offers (we could have passed the event too)
}
