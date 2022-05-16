extern crate serde_json;
use reqwest::*;
use std::{collections::HashMap};
use serde_json::{Value, json};
use tokio;
use serde::{Serialize, Deserialize};
use futures::executor::block_on;
use crate::encryption::decrypt_string;


#[derive(Debug, Serialize, Deserialize)]
pub struct messages {
    pub messages: Vec<String>,
    pub senders: Vec<String>
}

pub fn decrypt_msgs(msgs: messages, key: String) -> messages {
    let mut decrypted_msgs = messages {
        messages: Vec::new(),
        senders: Vec::new()
    };
    for ms in msgs.senders {
        let k = String::from(&key);
        let dec = decrypt_string(k, ms);
        decrypted_msgs.senders.push(dec);
    }
    for ms in msgs.messages {
        let k = String::from(&key);
        let dec = decrypt_string(k, ms);
        decrypted_msgs.messages.push(dec);
    }
    decrypted_msgs
}



pub async fn get_msgs_encrypted(server: String, contact: String) -> messages {
    let url: String = server + "/" + &contact;
    // Create a get request to the server
    let client = reqwest::Client::new();
    let r = client.get(url).send().await;
    match r {
        Ok(mut response) => {
            let body = response.text().await.unwrap();
            let json: Value = serde_json::from_str(&body).unwrap();
            let messages: Vec<String> = serde_json::from_value(json["messages"].clone()).unwrap();
            let senders: Vec<String> = serde_json::from_value(json["senders"].clone()).unwrap();
            let msgs = messages {
                messages: messages,
                senders: senders
            };
            return msgs;
            }
        
        Err(e) => {
            println!("{}", e);
            let errmsgs = messages {
                messages: vec!["QUARZ_ERRORWHILEPARSINGMESSAGES".to_string()],
                senders: vec!["QUARZ_ERRORWHILEPARSINGMESSAGES".to_string()]
            };
            return errmsgs;
            }
        }
    }

#[allow(dead_code)]
pub async fn send_msg(server: String, msg: String, key: String, contact: String, sender: String) -> bool {
    // Returns true if everything went well
    let message = crate::encryption::encrypt_string(key.clone(), msg);
    let client = Client::new();
    let adress = crate::encryption::encrypt_string(key.clone(), contact);
    let author = crate::encryption::encrypt_string(key.clone(), sender);
    // Create a proper .JSON object
    let mut data = HashMap::new();
    data.insert("message", message);
    data.insert("contact", adress);
    data.insert("sender", author);
    // Send the message
    let res = client.post(server)
        .json(&data)
        .send()
        .await
        .unwrap();
    if res.status().is_success() {
        return true;
    } else {
        return false;
    }
}
