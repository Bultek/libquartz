use reqwest::*;
use std::{collections::HashMap};

#[allow(dead_code)]
pub async fn send_msg(server: String, msg: String, key: String, contact: String, sender: String) -> bool {
    // Returns true if everything went well
    let message = crate::encryption::encrypt_string(key.clone(), msg);
    let client = Client::new();
    let author = crate::encryption::encrypt_string(key.clone(), sender);
    // Create a proper .JSON object
    let mut data = HashMap::new();
    data.insert("message", message);
    data.insert("contact", contact);
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
