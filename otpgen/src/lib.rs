use crypto::digest::Digest;
use crypto::sha2::Sha256;
use curl::easy::Easy;
use curl::easy::List;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::io::Read;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub otp: String,
}

pub fn generate_otp(secret: &[u8; 32], time: u64) -> u32 {
    let mut hasher = Sha256::new();
    let rounded_time: u64 = time - (time % 30);
    hasher.input(&rounded_time.to_ne_bytes());
    hasher.input(secret);
    let mut result = [0u8; 32];
    hasher.result(&mut result);

    u32::from_ne_bytes(result[0..4].try_into().unwrap()) % 10000
}

pub fn otp_now(secret: &[u8; 32]) -> u32 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    generate_otp(secret, now)
}

pub fn send_auth_request(req: &AuthRequest, host: &str) -> u32 {
    let json = serde_json::to_string(req).unwrap();
    let mut data = json.as_bytes();

    let mut easy = Easy::new();
    easy.url(&format!("http://{}:8000/auth", host)).unwrap();
    easy.post(true).unwrap();

    let mut list = List::new();
    list.append("content-type: application/json").unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    easy.http_headers(list).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
            .unwrap();
        transfer.perform().unwrap();
    }
    easy.response_code().unwrap()
}
