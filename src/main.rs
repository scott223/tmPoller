use std::{error::Error, io::{self, Stdout},};
use ratatui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{self, KeyCode, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::*, prelude::*, widgets::*};

use anyhow::{Context, Result};

// main function
// this function will initialize and variables needed, and call the main loop function (located in lib.rs). after loop is finished, do cleanup

fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let mut tm_events = vec![
        tm_poller::schema::TMEvent::new(
            "284753".to_string(),
        ),
        tm_poller::schema::TMEvent::new(
            // Lowlands
            "280409".to_string(),
        ), 
    ];

    let mut terminal = tm_poller::setup_terminal().context("`terminal setup failed")?;
    
    println!("Initialized, running program ...");

    // Run the main loop
    tm_poller::run(&mut terminal, &mut tm_events).unwrap_or_else(|err| {
        panic!("Main loop error: {}", err) //if something goes wrong in the main loop, panic! as this is an unknown issue
    });

    // Preparing for exit

    tm_poller::restore_terminal(&mut terminal).context("restore terminal failed")?;

    println!("Quiting, bye!");
    Ok(())
}
