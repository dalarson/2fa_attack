#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use otpgen::AuthRequest;
use rocket::State;

use otpgen::otp_now;
use rocket_contrib::json::Json;
use std::sync::Mutex;

use crypto::aessafe::AesSafe256Decryptor;
use crypto::symmetriccipher::BlockDecryptor;

struct OtpState {
    pub secret: [u8; 32],
    pub last_otp: Mutex<u32>,
}

use rocket::http::Status;

#[post("/auth", data = "<req>")]
fn auth(state: State<OtpState>, req: Json<AuthRequest>) -> Status {
    // println!("Got an auth request: {:?}", request);
    println!("___________________\nNEW AUTH REQUEST RECEIVED");
    let mut last_otp = state.last_otp.lock().unwrap();
    // Generate our code
    let generated_code = otp_now(&state.secret);

    if generated_code == *last_otp && std::env::var("OTP_DO_NOT_BURN").is_err() {
        println!("Current code has expired. Please wait for next code.");
        return Status::from_code(401).unwrap();
    }

    println!("Expected code: {}", generated_code);
    *last_otp = generated_code;

    // Pull out the code given to us by the client and decrypt it
    let decryptor = AesSafe256Decryptor::new(&state.secret);
    let given_code = hex::decode(&req.otp).unwrap();
    let mut decrypted_code_bytes = [0u8; 16];
    decryptor.decrypt_block(&given_code, &mut decrypted_code_bytes);
    let decrypted_code = u128::from_ne_bytes(decrypted_code_bytes);
    println!("Decrypted code: {}", decrypted_code);

    if decrypted_code as u32 == generated_code {
        println!("Authentication successful.");
        return Status::from_code(200).unwrap();
    } else {
        println!("Authentication denied.");
        return Status::from_code(401).unwrap();
    }
}

fn main() {
    let state = OtpState {
        secret: *include_bytes!("shared-secret"),
        last_otp: Mutex::new(0),
    };
    rocket::ignite()
        .mount("/", routes![auth])
        .manage(state)
        .launch();
}
