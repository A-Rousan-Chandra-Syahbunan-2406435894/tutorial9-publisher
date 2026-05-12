use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, _m: Box<UserCreatedEventMessage>) -> Result<(), HandleError> { Ok(()) }
    fn get_handler_action(&self) -> String { "".to_string() }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher("amqps://ojzbiupq:BQhrSm_-y4CCijnHLuB9gyOzzyuZPgXp@armadillo.rmq.cloudamqp.com/ojzbiupq".to_owned()).unwrap();
    let names = vec!["Amir", "Budi", "Cica", "Dira", "Emir"];
    for (i, name) in names.iter().enumerate() {
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
            user_id: (i + 1).to_string(), 
            user_name: format!("2406435894-{}", name) 
        });
    }
    println!("Selesai ngirim bang!");
}