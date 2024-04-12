use std::collections::{HashMap, VecDeque};
use tokio::sync::mpsc::{Sender, Receiver};

use super::messages::{MessageType, StateActorMessage};

#[derive(Debug)]
pub struct StateActor {
    pub chat_queue: VecDeque<i32>,
    pub chat_logs: HashMap<i32, Vec<String>>,
    pub receiver: Receiver<StateActorMessage>,
    pub sender: Sender<StateActorMessage>
}

impl StateActor {
    pub fn new(receiver: Receiver<StateActorMessage>, sender: Sender<StateActorMessage>) -> Self {
        Self { chat_queue: VecDeque::new(), chat_logs: HashMap::new(), receiver, sender }
    }

    pub fn get_message_data(&mut self, chat_id:i32) -> Vec<String> {
        self.chat_logs.remove(&chat_id).unwrap()
    }

    pub fn insert_message(&mut self, chat_id: i32, message_data: String) {
        match self.chat_logs.get_mut(&chat_id) {
            Some(chat_log) => {
              chat_log.push(message_data);
            },
            None => {
                self.chat_queue.push_back(chat_id.clone());
                self.chat_logs.insert(chat_id, vec![message_data]);
            }
        }
    }

    async fn handle_message(&mut self, message: StateActorMessage) {
        println!("state actor received a message");

        match message.message_type {
            MessageType::INPUT => {
                self.insert_message(message.chat_id.unwrap(), message.single_data.unwrap())
            },
            MessageType::OUTPUT => {
                match self.chat_queue.pop_front() {
                    Some(chat_id) => {
                        let data = self.get_message_data(chat_id.clone());
                        let message = StateActorMessage {
                            message_type: MessageType::OUTPUT,
                            chat_id: Some(chat_id),
                            single_data: None,
                            block_data: Some(data),
                        };

                        let _ = self.sender.send(message).await.unwrap();
                    },
                    None => {
                        let message = StateActorMessage {
                            message_type: MessageType::EMPTY,
                            chat_id: None,
                            single_data: None,
                            block_data: None,
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    }
                }
            },
            MessageType::EMPTY => panic!("empty messages should not be sent to the state actor")
        }

        println!("{:?}", self.chat_logs);
        println!("{:?}", self.chat_queue)
    }

    pub async fn run(mut self) {
        println!("state actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
