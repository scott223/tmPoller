use chrono::{DateTime, Local};

// a single TicketMaster event
#[derive(Debug)]
pub struct TMEvent {
    pub id: String,
    pub num_offers: usize,
    pub last_updated: DateTime<Local>,
    pub last_update_status_code : reqwest::StatusCode,
}

impl TMEvent {
    pub fn new(a_id: String) -> Self {

        let new_self = Self {
            id: a_id,
            num_offers: 0 as usize,
            last_updated: Local::now(),
            last_update_status_code : reqwest::StatusCode::CONTINUE,
        };

        return new_self

    }
}

pub struct App {
    pub events: Vec<TMEvent>,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            events: vec![
                TMEvent::new(
                    "284753".to_string(),
                ),
                TMEvent::new(
                    // Lowlands
                    "280409".to_string(),
                ), 
            ],
            messages: Vec::new(),
        }
    }

}

impl App {
    pub fn submit_message(&mut self, input: &str) {
        self.messages.push(input.to_string().clone());
    }
}
