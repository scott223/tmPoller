use actix_web::{web, App, HttpServer};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

use tm_poller::run;

// Main function
// This function will initialize and variables needed, and set up three Tokio worker tasks
// 1) a small actix http server so we can get results and check if the program is working through a basic API
// 2) the main loop function (located in lib.rs), this loop looks for keystrokes, executings the polling at a set interval and draws a UI
// 3) a helper worker that checks if any shutdown signal is sent by the main loop, so it can inform the http server to close down

#[tokio::main(flavor = "multi_thread", worker_threads = 2)] // as an experiment, set the maximum number of workers
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events through the default function
    // Wrap the app into an Arc (to share between treads) and Mutex (so we can lock if we need to access)
    let app = Arc::new(Mutex::new(tm_poller::schema::App::default()));

    // Create a seperate variable for messages (more like a log) - these are show in the UI
    let msgs: tm_poller::schema::Messages = tm_poller::schema::Messages::default();

    // Create a tokio::mpsc challen to send and recevie the the shutdown signal across workers
    let (tx, mut rx) = mpsc::channel(32);

    // set the worker task for the main wloop worker
    let worker_task = tokio::spawn(run(tx, app.clone(), msgs));

    // Create the server and give a clone of the Arc<Mutex<App>>> as a reference
    let server = HttpServer::new(move || {
        App::new()
            .service(tm_poller::poller_http::return_json)
            .app_data(web::Data::new(app.clone()))
    })
    .bind("127.0.0.1:8080")?
    // disable default signal handling
    .disable_signals()
    .workers(2)
    .run(); //configure the http server

    let server_handle = server.handle(); //set a handle for the server
    let server_task = tokio::spawn(server); //spawn the server as a tokio worker

    // Spawn another tokio worker to handle the shutdown once a signal is received
    let shutdown = tokio::spawn(async move {
        // Listen for shutdown signal
        while let Some(_shutdown_signal) = rx.recv().await { //wait untill we have a shutdown signal from on of the workers
        }
        let server_stop = server_handle.stop(true);

        // Await shutdown of tasks
        server_stop.await;
    });

    // Wait for all the workers to finish
    let _ = tokio::try_join!(server_task, worker_task, shutdown).expect("unable to join tasks");

    // Exit
    println!("Quiting, bye!");
    Ok(())
} // fn main
