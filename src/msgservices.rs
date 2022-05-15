use reqwest::*;
use std::{collections::HashMap};

#[allow(dead_code)]
pub async fn send_msg(server: String, msg: String, key: String, contact: String) -> bool {
    // Returns true if everything went well
    let message = crate::encryption::encrypt_string(key, msg);
    let client = Client::new();
    // Create a proper .JSON object
    let mut data = HashMap::new();
    data.insert("message", message);
    data.insert("contact", contact);
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
