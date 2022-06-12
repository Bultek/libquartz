use rand::{self, random};
use std::{env, fs, path};

#[allow(unused_attributes)]
#[no_mangle]
#[allow(dead_code)]
fn rand_string(length: i32) -> String {
    (0..length)
        .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
        .collect()
}

#[allow(unused_attributes)]
#[no_mangle]
#[allow(dead_code)]
pub fn check_if_key_is_valid(key: &str) -> bool {
    if key.len() != 45 {
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
        false
    } else {
        true
    }
}

#[allow(unused_attributes)]
#[no_mangle]
pub fn get_default_key() -> String {
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();
    let cfgpath = path::Path::new(&home).join(".config").join("libquartz");
    let out = fs::read_to_string(cfgpath.join("defaultkey"));
    match out {
        Ok(out) => out,
        Err(_) => {
            panic!("NO DEFAULT KEY SET! PLEASE INSTALL QUARTZCTL AND SET ONE")
        }
    }
}

#[allow(unused_attributes)]
#[no_mangle]
pub fn gen_key() -> String {
    let mut key: String = rand_string(45);
    let mut is_valid = check_if_key_is_valid(&key);
    while !is_valid {
        key = rand_string(45);
        is_valid = check_if_key_is_valid(&key);
    }
    key
}
