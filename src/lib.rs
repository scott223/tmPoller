use std::{
    error::Error,
    io::Stdout,
    time::Duration,
    time::Instant,
};

use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::{backend::*, prelude::*};

use anyhow::Result;

pub mod schema;
pub mod poller_ui;
mod event_poller;

const DEFAULT_POLLING_INTERVAL: Duration = Duration::new(30, 0);

// Function run
//
// creates an "infitine" loop that checks for keystrokes, and executes the correspondig functions
//
// Arguments
//
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: schema::App,
) -> Result<(), Box<dyn Error>> {
    let mut last_update = Instant::now();
    let poll_on_interval = DEFAULT_POLLING_INTERVAL;

    app.submit_message("Initialized, running program ...");

    app.submit_message("Executing first poll");

    match event_poller::update_events(&mut app) {
        // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
        Ok(()) => {
            last_update = Instant::now();
            app.submit_message("All events updated");
        }
        Err(e) => eprintln!("Error with updating events: {}", e),
    }    
    // Running main loop
    'mainloop: loop {
        terminal.draw(|f| poller_ui::ui(f, &app))?;

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // User pressed ESC or 'q', breaking the main loop
                        break 'mainloop;
                    }
                    KeyCode::Char('p') => {
                        // User pressed 'p', forcing an update of the events
                        app.submit_message("User forced polling of all events");

                        match event_poller::update_events(&mut app) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(()) => {
                                last_update = Instant::now();
                                app.submit_message("All events updated");
                            }
                            Err(e) => eprintln!("Error with updating events: {}", e),
                        }
                        // println!("Data dump: {:?}", app.events); // temp: data dump
                    }
                    _ => {}
                }
            }
        }

        if Instant::now().duration_since(last_update) > poll_on_interval {
            // Update interval exceeded
            app.submit_message("Interval triggered polling of all events");

            match event_poller::update_events(&mut app) {
                // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                Ok(()) => {
                    last_update = Instant::now();
                    app.submit_message("All events updated")
                }
                Err(e) => eprintln!("Error: {}", e),
            }
            // println!("Data dump: {:?}", app.events); // temp: data dump
        }
    }

    Ok(())
} // fn run