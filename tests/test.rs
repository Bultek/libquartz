//use colored::*;
use futures::executor::block_on;
use libquartz::*;
use std::{env, fs::*};
use tokio;

#[tokio::main]
#[test]
async fn main() {
    // Check if ~/.qkey exists
    #[allow(deprecated)]
    let home_dir = env::home_dir().unwrap();
    let qkey_path = home_dir.join(".qkey");
    if metadata(&qkey_path).is_err() {
        // Create ~/.qkey
        let key = keytools::gen_key();
        std::fs::write(&qkey_path, key).unwrap();
    }

    let qkey = qkey_path.into_os_string().into_string().unwrap();
    let key = std::fs::read_to_string(qkey).unwrap();
    if keytools::check_if_key_is_valid(&key) == false {
        panic!("Key isn't valid");
    }
    let k = key.clone();
    let msg = encryption::encrypt_string(String::from(&key), "Hello World!".to_string());
    let result = msgservices::send_msg(
        "http://127.0.0.1:8000/incoming".to_string(),
        msg,
        k.clone(),
        "test".to_string(),
        "tester".to_string(),
    );
    let r = block_on(result);
    println!("After r");
    println!("{}", r);
    let result = msgservices::send_msg(
        "http://127.0.0.1:8000/incoming".to_string(),
        "Hello World!".to_string(),
        k.clone(),
        "Hello".to_string(),
        "World".to_string(),
    );
    let b = block_on(result);
    println!("{}", b);
    // Recieve messages and print
    let a = msgservices::get_msgs_encrypted(
        "http://127.0.0.1:8000/messages".to_string(),
        encryption::encrypt_string(k.clone(), "Hello".to_string()),
    );
    let o = block_on(a);
    let d = msgservices::decrypt_msgs(o, k.clone());
    for i in 0..d.messages.len() {
        println!("{} - {}", d.senders[i], d.messages[i]);
    }
}
