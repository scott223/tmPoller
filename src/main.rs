pub mod schema;
use std::error::Error;

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

    // Run the main loop
    tm_poller::run(&mut tm_events).unwrap_or_else(|err| {
        panic!("Main loop error: {}", err) //if something goes wrong in the main loop, panic! as this is an unknown issue
    });

    // Preparing for exit
    println!("Quiting, bye!");
    Ok(())
}
