use actix_web::{get, web};
use std::sync::{Arc, Mutex};

#[get("/json/")]
async fn return_json(
    data: web::Data<Arc<Mutex<crate::schema::App>>>,
) -> actix_web::Result<web::Json<crate::schema::App>> {
    let app_unlocked = data.lock().unwrap(); //get a lock on the variable
    Ok(web::Json(app_unlocked.clone())) //return a JSON
} // fn
