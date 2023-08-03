use std::{
    error::Error,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode}, 
    terminal,
};

use reqwest::{
    Body,
    blocking,
};

use serde::Deserialize;

const EVENT_ID: &str = "284753"; // Lowlands: 280409
const BASE_URL: &str = "https://availability.ticketmaster.eu/api/v2/TM_NL/resale/";

#[derive(Deserialize, Debug)]
struct TMResponse {
    offers: Vec<Offer>,
}

#[derive(Deserialize, Debug)]
struct Offer {
    id: String,
}

fn main() -> Result<(),Box<dyn Error>> {

    println!("Initialized, running program ...");

    'mainloop: loop {

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {

                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'mainloop
                    }
                    KeyCode::Char('p') => {
                        
                        match poll_event() {
                            Ok(n) => println!("Succesfully polled"),
                            Err(err) => println!("Error: {}", err),
                        }
                    }
                    _ => {}
                }
            }
        }

    }

    println!("Quiting, bye!");
    Ok(())

}

 fn poll_event() -> Result<(), Box<dyn Error>> {

    let request_url = format!("{}{}",BASE_URL,EVENT_ID);
    println!("Polling event {}, url: {}", EVENT_ID, request_url);

    let resp_result = reqwest::blocking::get(request_url)?;
    let resp: TMResponse = resp_result.json()?;

    println!("There are {} offers for event {}",resp.offers.len(), EVENT_ID);

    Ok(())
}
