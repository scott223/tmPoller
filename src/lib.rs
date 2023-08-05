use crossterm::event::{self, Event, KeyCode};
use std::{error::Error, time::Duration};
mod event_poller;
pub mod schema;

use crate::schema::TMEvent;

// Function run
//
// creates an "infitine" loop that checks for keystrokes, and executes the correspondig functions
//
// Arguments
//
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)

pub fn run(tm_events: &mut Vec<TMEvent>) -> Result<(), Box<dyn Error>> {
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

                        match event_poller::update_events(tm_events) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(()) => println!("All events updated"),
                            Err(e) => println!("Error: {}", e),
                        }
                        println!("Data dump: {:?}", tm_events); // temp: data dump
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
