use chrono::{DateTime, Local};

// a single TicketMaster event
#[derive(Debug)]
pub struct TMEvent {
    pub id: String,
    pub name: String,
    pub num_offers: usize,
    pub last_updated: DateTime<Local>,
    pub last_update_status_code : reqwest::StatusCode,
}

impl TMEvent {
    pub fn new(a_id: String, a_name: String) -> Self {

        let new_self = Self {
            id: a_id,
            name: a_name,
            num_offers: 0 as usize,
            last_updated: Local::now(),
            last_update_status_code : reqwest::StatusCode::CONTINUE,
        };

        return new_self

    }
}

pub struct App {
    pub events: Vec<TMEvent>,
    pub messages: Vec<Message>,
}

impl Default for App {
    fn default() -> App {
        App {
            events: vec![
                TMEvent::new(
                    "284753".to_string(),
                    "Dynamo Metalfest".to_string(),
                ),
                TMEvent::new(
                    // Lowlands
                    "280409".to_string(),
                    "Lowlands".to_string(),
                ),
                TMEvent::new(
                    // Dummy
                    "280407".to_string(),
                    "Dummy".to_string(),
                ),  
            ],
            messages: Vec::new(),
        }
    }

}

impl App {
    pub fn submit_message(&mut self, input: &str) {
        let new_message = Message::new(input.to_string().clone());
        self.messages.push(new_message);
    }
}

pub struct Message {
    pub content: String,
    pub datetime_sent: DateTime<Local>,
}

impl Message {
    pub fn new(msg: String) -> Self {
        let new_message: Message = Self {
            content: msg,
            datetime_sent: Local::now(),
        };
        return new_message
    }
}
