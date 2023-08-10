use std::error::Error;
use tokio::sync::mpsc;

use actix_web::{get, web, App, HttpServer, Responder};

use anyhow::{Result};
use tm_poller::run;

// main function
// This function will initialize and variables needed, and call the main loop function (located in lib.rs). after loop is finished, do cleanup
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let app = tm_poller::schema::App::default();
    let (tx, mut rx) = mpsc::channel(32); //create a channel to send shutdown signal

    let server = HttpServer::new(|| App::new().service(greet))
        .bind("127.0.0.1:8080")?
        // disable default signal handling
        .disable_signals()
        .workers(2)
        .run(); //configure the http server

    let server_handle = server.handle(); //set a handle for the server

    let server_task = tokio::spawn(server); //spawn the server as a tokio worker
    let worker_task = tokio::spawn(run(tx, app)); //spawn the main app as a tokio worker

    let shutdown = tokio::spawn(async move { //spwan another tokio worker to handle the shutdown once a signal is received
        // listen for shutdown signal
        while let Some(shutdown_signal) = rx.recv().await { //wait untill we have a shutdown signal from on of the workers

        }

        let server_stop = server_handle.stop(true); // stop the http server
        // await shutdown of tasks
        server_stop.await;
    });

    let _ = tokio::try_join!(server_task, worker_task, shutdown).expect("unable to join tasks"); //wait for all the workers to finish

    // Preparing for exit
    println!("Quiting, bye!");
    Ok(())
} // fn main

#[get("/hello/{name}")] // to be moved to seperate module
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}