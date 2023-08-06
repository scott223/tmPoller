use std::error::Error;

// main function
// this function will initialize and variables needed, and call the main loop function (located in lib.rs). after loop is finished, do cleanup
fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let mut tm_events = vec![
        tm_poller::schema::TMEvent::new(
            "284753".to_string(),
            0,
        ),
        tm_poller::schema::TMEvent::new(
            // Lowlands
            "280409".to_string(),
            0,
        ),
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
