
    use colored::*;
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    #[allow(dead_code)]
    pub fn encrypt_string(key: String, text: String) -> String {
        let mc = new_magic_crypt!(key, 256);
        let encrypted = mc.encrypt_str_to_base64(text);
        return encrypted;
    }
    #[allow(dead_code)]
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
    }

