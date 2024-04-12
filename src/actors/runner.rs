use super::messages::{MessageType, StateActorMessage};
use tokio::sync::mpsc::{Sender, Receiver};

use std::time;

pub struct RunnerActor {
    pub interval: i32,
    pub receiver: Receiver<StateActorMessage>,
    pub sender: Sender<StateActorMessage>
}


impl RunnerActor {
    pub fn new(interval: i32, receiver: Receiver<StateActorMessage>, sender: Sender<StateActorMessage>) -> Self {
        Self { interval, receiver, sender }
    }

    pub async fn run(mut self) {
        println!("runner actor is running");
        let seconds = time::Duration::from_secs(self.interval as u64);

        loop {
            tokio::time::sleep(seconds).await;
            let message = StateActorMessage {
                message_type: MessageType::OUTPUT,
                chat_id: None,
                single_data: None,
                block_data: None,
            };
            match self.sender.send(message).await {
                Ok(_) => {
                    let message = self.receiver.recv().await.unwrap();
                    match message.message_type {
                        MessageType::OUTPUT => {
                          message.send_to_server().await;
                        },
                        _ => println!("state is empty")
                    }
                },
                Err(_) => println!("runner failed to send message")
            }
        }
    }
}
