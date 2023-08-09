use chrono::{DateTime, Local};

// Main App state, that holds the events and messages
pub struct App {
    pub events: Vec<TMEvent>,
    pub messages: Vec<Message>,
}

// default adds some events to the App state for testing
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

// a single TicketMaster event
#[derive(Debug)]
pub struct TMEvent {
    pub id: String,
    pub name: String,
    pub num_offers: usize,
    pub last_updated: DateTime<Local>,
    pub last_update_status_code : reqwest::StatusCode,
}

// Constructor like function to create a new TM event
impl TMEvent {
    pub fn new(a_id: String, a_name: String) -> Self {
        let new_self = Self {
            id: a_id,
            name: a_name,
            num_offers: 0 as usize,
            last_updated: Local::now(),
            last_update_status_code : reqwest::StatusCode::CONTINUE, // now picked this random status code as a starting code
        };

        return new_self
    }
}

// Functions to change the App state, for now only for adding a new message
impl App {
    pub fn submit_message(&mut self, input: &str) {
        let new_message = Message::new(input.to_string().clone());
        self.messages.push(new_message);
    }
}

// a single message that holds a string and a DateTime
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