use centichain_keypair::CentichainKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub key: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct Keys {
    pub private: String,
    pub public: String,
}

#[tauri::command]
pub fn check_key(pkey: String) -> Response {
    match CentichainKey::check_phrase(&pkey) {
        Ok(public) => {
            let response = Response {
                key: true,
                status: public.to_string(),
            };
            response
        }
        Err(_e) => {
            let response = Response {
                key: false,
                status: "The private key you entered is incorrect!".to_string(),
            };
            response
        }
    }
}

#[tauri::command]
pub fn generate_keys() -> Keys {
    let keys = CentichainKey::generate();
    let response = Keys {
        private: keys.0,
        public: keys.1.to_string(),
    };
    response
}
