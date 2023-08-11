use std::error::Error;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

use actix_web::{get, web, App, HttpServer, Responder};

use anyhow::{Result};
use tm_poller::{run, schema::{Messages, Message}};

// main function
// This function will initialize and variables needed, and call the main loop function (located in lib.rs). after loop is finished, do cleanup
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initiliazing main variable holding the events, and pre-filling it with two events
    let app = Arc::new(Mutex::new(tm_poller::schema::App::default())); //wrap the app into an Arc (to share between treads) and Mutex (so we can lock if we need to access)
    let msgs: tm_poller::schema::Messages = tm_poller::schema::Messages::default(); //create a seperate variable for messages (more like a log)
    let (tx, mut rx) = mpsc::channel(32); //create a channel to send shutdown signal
    let worker_task = tokio::spawn(run(tx, app.clone(), msgs)); //spawn the main app as a tokio worker
    let server = HttpServer::new(move|| App::new().service(greet).app_data(web::Data::new(app.clone()))) //create the server and give a clone of the Arc<Mutex<App>>> as a reference
        .bind("127.0.0.1:8080")?
        // disable default signal handling
        .disable_signals()
        .workers(2)
        .run(); //configure the http server

    let server_handle = server.handle(); //set a handle for the server

    let server_task = tokio::spawn(server); //spawn the server as a tokio worker
 
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

#[get("/json/")] // to be moved to seperate module
async fn greet(data: web::Data<Arc<Mutex<tm_poller::schema::App>>>) -> actix_web::Result<web::Json<tm_poller::schema::App>>{
    let app_unlocked = data.lock().unwrap(); //get a lock on the variable
    Ok(web::Json(app_unlocked.clone())) //return a JSON
    //format!("Hello {}! timestamp of first event is {:?}", name, app_unlocked.events[0].last_updated)
}