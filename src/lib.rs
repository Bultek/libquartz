pub mod keytools
{
    use colored::*;
    use rand::{self, random};

    pub fn check_equality(key1: String, key2: String) -> bool {
        if key1==key2 {
            return true;
        }
        else {
            return false;
        }
    }


    fn rand_string(length: i32) -> String {
        (0..length).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
    }

    pub fn check_if_key_is_valid(key: &str) -> bool {
        if key.len() != 45 {
            println!("{}{}{}","Key isn't valid: ".bright_red(), key.len()," characters long.".bright_red());
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
        if nums == 9 {
            return true;
        } else {
            return false;
        }
    }
    pub fn gen_key() -> String {
        let mut key: String = rand_string(45);
        let mut is_valid = check_if_key_is_valid(&key);
        while is_valid==false {
            key = rand_string(45);
            is_valid = check_if_key_is_valid(&key);
        }
        return key;
    }
}

pub mod encryption {
    use colored::*;
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    pub fn encrypt_string(key: String, text: String) -> String {
        let mc = new_magic_crypt!(key, 256);
        let encrypted = mc.encrypt_str_to_base64(text);
        return encrypted;
    }
    pub fn decrypt_string(key: String, encrypted_data: String) -> String {
        let mc = new_magic_crypt!(key, 256);
        let decrypted = mc.decrypt_base64_to_string(encrypted_data);
        match decrypted {
            Err(e) => {
                println!("Detailed Error: {}", e.to_string().bright_red());
                return "!!!Invalid Key!!!".to_string();
            }
            Ok(s) => {
                return s;
            }
        }
        //return decrypted;
    }
}

pub mod msgservices {}
