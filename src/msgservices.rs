extern crate serde_json;
use crate::encryption::decrypt_string;
use reqwest::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[allow(unused_attributes)]
#[no_mangle]
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct messages {
    pub contact: Vec<String>,
    pub messages: Vec<String>,
    pub senders: Vec<String>,
}
#[allow(unused_attributes)]
#[no_mangle]
#[allow(non_camel_case_types)]
pub struct messages_nocontacts {
    pub messages: Vec<String>,
    pub senders: Vec<String>,
}

#[allow(unused_attributes)]
#[no_mangle]
impl messages_nocontacts {
    pub fn from(msgs: messages) {
        let mut _m: messages_nocontacts = messages_nocontacts {
            senders: msgs.senders,
            messages: msgs.messages,
        };
    } 
}
#[allow(unused_attributes)]
#[no_mangle]
pub fn decrypt_msgs(msgs: messages_nocontacts, key: String) -> messages_nocontacts {
    let mut decrypted_msgs = messages_nocontacts {
        messages: Vec::new(),
        senders: Vec::new(),
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
#[allow(unused_attributes)]
#[no_mangle]
pub async fn get_msgs_encrypted(server: String, contact: String) -> messages_nocontacts {
    // Create a get request to the server
    let client = reqwest::Client::new();
    let r = client.get(server).send().await;
    match r {
        Ok(response) => {
            let body = response.text().await.unwrap();
            let json: Value = serde_json::from_str(&body).unwrap();
            let messages: Vec<String> = serde_json::from_value(json["messages"].clone()).unwrap();
            let senders: Vec<String> = serde_json::from_value(json["senders"].clone()).unwrap();
            let contacts: Vec<String> = serde_json::from_value(json["contacts"].clone()).unwrap();
            let msgs = messages {
                contact: contacts,
                messages: messages,
                senders: senders,
            };
            println!("{}", &body);
            let mut _m: messages_nocontacts = messages_nocontacts {
                senders: Vec::new(),
                messages: Vec::new(),
            };
            // remove useless messages

            for i in 0..msgs.messages.len() {
                if msgs.contact[i] == contact {
                        _m.messages.push(msgs.messages[i].clone());
                        _m.senders.push(msgs.senders[i].clone());
                }
            }

            return _m;
        }

        Err(e) => {
            println!("{}", e);
            let errmsgs = messages_nocontacts {
                messages: vec!["QUARTZ_ERRORWHILEPARSINGMESSAGES".to_string()],
                senders: vec!["QUARTZ_ERRORWHILEPARSINGMESSAGES".to_string()],
            };
            return errmsgs;
        }
    }
}

#[no_mangle]
#[allow(dead_code)]
pub async fn send_msg(
    server: String,
    msg: String,
    key: String,
    contact: String,
    sender: String,
) -> bool {
    // Returns true if everything went well
    let message = crate::encryption::encrypt_string(key.clone(), msg);
    let client = Client::new();
    let address = crate::encryption::encrypt_string(key.clone(), contact);
    let author = crate::encryption::encrypt_string(key.clone(), sender);
    // Create a proper .JSON object
    let mut data = HashMap::new();
    data.insert("message", message);
    data.insert("contact", address);
    data.insert("sender", author);
    // Send the message
    let res = client.post(server).json(&data).send().await.unwrap();
    if res.status().is_success() {
        return true;
    } else {
        return false;
    }
}
