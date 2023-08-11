use std::{
    error::Error,
    io::Stdout,
    time::Duration,
    time::Instant,
};

use std::sync::{
    atomic::{AtomicBool, Ordering},
};

use std::sync::{Arc, Mutex};

use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::{backend::*, prelude::*};

use anyhow::{Context, Result};

pub mod schema;
pub mod poller_ui;
mod event_poller;

use schema::Messages;
use tokio::sync::mpsc;

use poller_ui::{setup_terminal,restore_terminal};

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
pub async fn run(
    mut shutdown_channel: mpsc::Sender<bool>,
    mut app: Arc<Mutex<schema::App>>,
    mut messages: schema::Messages,
) -> std::io::Result<()> {
    let mut last_update = Instant::now();
    let poll_on_interval = DEFAULT_POLLING_INTERVAL;
    let mut terminal = match setup_terminal() {
        Ok(t) => t,
        Err(error) => panic!("Error {}",error)
    };

    messages.submit_message("Initialized, running program ...");
    messages.submit_message("Executing first poll");

    match event_poller::update_events(&mut app) {
        // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
        Ok(()) => {
            last_update = Instant::now();
            messages.submit_message("All events updated");
        }
        Err(e) => eprintln!("Error with updating events: {}", e),
    }
    // Running main loop
    'mainloop: loop {
        terminal.draw(|f| poller_ui::ui(f, &app, &messages))?;
        while event::poll(Duration::default())? {

            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // User pressed ESC or 'q', breaking the main loop
                        break 'mainloop;
                    }
                    KeyCode::Char('p') => {
                        // User pressed 'p', forcing an update of the events
                        messages.submit_message("User forced polling of all events");

                        match event_poller::update_events(&mut app) {
                            // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                            Ok(()) => {
                                last_update = Instant::now();
                                messages.submit_message("All events updated");
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
            messages.submit_message("Interval triggered polling of all events");

            match event_poller::update_events(&mut app) {
                // running update function & print a message for feedback. passing the main variable as mutable borrow, so the function can actually change the variable
                Ok(()) => {
                    last_update = Instant::now();
                    messages.submit_message("All events updated")
                }
                Err(e) => eprintln!("Error: {}", e),
            }
            // println!("Data dump: {:?}", app.events); // temp: data dump
        }
    }

    poller_ui::restore_terminal(&mut terminal).context("restore terminal failed"); //TODO handle a possible error
    shutdown_channel.send(true).await.expect("cannot send shutdown message");
    Ok(())
} // fn run