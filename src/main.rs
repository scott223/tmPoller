pub mod schema;
use std::{error::Error};

// TicketMaster API: full response (JSON deserialized)

fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let mut tm_events = vec![
        tm_poller::schema::TMEvent {
            // Some random event that has tickets
            id: "284753".to_string(),
            num_offers: 0,
        },
        tm_poller::schema::TMEvent {
            // Lowlands
            id: "280409".to_string(),
            num_offers: 0,
        },
    ];
    println!("Initialized, running program ...");

    tm_poller::run(&mut tm_events).unwrap_or_else(|err| {
        panic!("Main loop error: {}", err) //if something goes wrong in the main loop, panic! as this is an unknown issue
    });

    // Preparing for exit
    println!("Quiting, bye!");
    Ok(())
}

// Function update_events
//
// Updates the events
//
// Arguments
//
// * tm_events - a vector of TMEvent that holds all the current events that need to be polled, and polling data gets added to this vector. note we need to keep ownership in the main function, and borrow ownership to the functions below
//
// Returns an Ok(()) if no errors and an Box<error> in case there is an (underlying error)


