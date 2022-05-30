use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[allow(unused_attributes)]
#[no_mangle]
#[allow(dead_code)]
pub fn encrypt_string(key: String, text: String) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_str_to_base64(text)
}

#[allow(unused_attributes)]
#[no_mangle]
#[allow(dead_code)]
pub fn decrypt_string(key: String, encrypted_data: String) -> String {
    let mc = new_magic_crypt!(key, 256);
    let decrypted = mc.decrypt_base64_to_string(encrypted_data);
    match decrypted {
        Err(_) => {
            "!!!Invalid Key!!!".to_string()
        }
        Ok(s) => {
            s
        }
    }
}
