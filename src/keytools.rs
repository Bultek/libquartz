use colored::*;
use rand::{self, random};



#[allow(dead_code)]
fn rand_string(length: i32) -> String {
    (0..length)
        .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
        .collect()
}

#[allow(dead_code)]
pub fn check_if_key_is_valid(key: &str) -> bool {
    if key.len() != 45 {
            println!(
                "{}{}{}",
                "Key isn't valid: ".bright_red(),
                key.len(),
                " characters long.".bright_red()
            );
            return false;
        }
        let mut nums: i32 = 0;
        for c in key.chars() {
            if c == '0'
                || c == '1'
                || c == '2'
                || c == '3'
                || c == '4'
                || c == '5'
                || c == '6'
                || c == '7'
                || c == '8'
                || c == '9'
            {
                nums += 1;
            }
        }
        if nums != 12 {
            println!(
                "{}{}{}",
                "Key isn't valid: ".bright_red(),
                nums,
                " numbers found.".bright_red()
            );
            return false;
        }
        else {
            return true;
        }
    }

pub fn gen_key() -> String {
    let mut key: String = rand_string(45);
    let mut is_valid = check_if_key_is_valid(&key);
    while is_valid == false {
        key = rand_string(45);
        is_valid = check_if_key_is_valid(&key);
    }
    return key;
}
