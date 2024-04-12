use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY
}

#[derive(Debug, Serialize)]
struct PostBody {
    pub chat_id: i32,
    pub block_data: String
}

#[derive(Debug, Serialize)]
pub struct StateActorMessage {
    pub message_type: MessageType,
    pub chat_id: Option<i32>,
    pub single_data: Option<String>,
    pub block_data: Option<Vec<String>>
}

impl StateActorMessage {
    pub async fn send_to_server(&self) {
        let lib_url = env::var("SERVER_URL").unwrap();
        let joined = self.block_data.clone().unwrap().join("$");
        let body = PostBody {
            chat_id: self.chat_id.unwrap(),
            block_data: joined
        };
        let client = reqwest::Client::new();
        let res = client.post(lib_url)
            .json(&body)
            .send()
            .await.unwrap();
        println!("{:?}", res);
    }
}
