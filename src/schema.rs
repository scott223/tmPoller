use chrono::{DateTime, Local};

// a single TicketMaster event
#[derive(Debug)]
pub struct TMEvent {
    pub id: String,
    pub num_offers: usize,
    pub last_updated: DateTime<Local>,
}

impl TMEvent {

    pub fn new(a_id: String, a_num_offers: usize) -> Self {

        let new_self = Self {
            id: a_id,
            num_offers: a_num_offers,
            last_updated: Local::now(),
        };

        return new_self

    }
    
}
