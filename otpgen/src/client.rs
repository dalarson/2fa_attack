use crypto::aessafe::AesSafe256Encryptor;
use crypto::symmetriccipher::BlockEncryptor;
use otpgen::{otp_now, send_auth_request, AuthRequest};
use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    // let mut client = OtpClient::connect().await?;

    let secret = include_bytes!("shared-secret");
    let encryptor = AesSafe256Encryptor::new(secret);

    let code = otp_now(&secret);

    println!("_________________________\nNEW AUTH REQUEST SENT");
    println!("Sending code: {}", code);

    let mut encrypted_code = [0u8; 16];
    encryptor.encrypt_block(&(code as u128).to_ne_bytes(), &mut encrypted_code);
    let otp = hex::encode(encrypted_code);
    let req = AuthRequest { otp };

    let response_code = send_auth_request(&req, &args[1]);
    let authenticated = 200 <= response_code && response_code < 300;
    println!(
        "{}",
        if authenticated {
            "Authentication successful!"
        } else {
            "Authentication FAILED"
        }
    );

    Ok(())
}
