use std::error::Error;
use anyhow::{Context, Result};

// main function
// This function will initialize and variables needed, and call the main loop function (located in lib.rs). after loop is finished, do cleanup

fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let app = tm_poller::schema::App::default();
    let mut terminal = tm_poller::setup_terminal().context("`terminal setup failed")?;

    // Run the main loop
    tm_poller::run(&mut terminal, app).unwrap_or_else(|err| {
        panic!("Main loop error: {}", err) //if something goes wrong in the main loop, panic! as this is an unknown issue
    });

    // Preparing for exit
    tm_poller::restore_terminal(&mut terminal).context("restore terminal failed")?;
    println!("Quiting, bye!");
    
    Ok(())
}