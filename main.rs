use std::env;
use std::process;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::Ipv4Addr;


const LOG_FILE: &str = "siren_capture.log"; 

fn main() {

        real_capture();
  
    println!("[+] Siren execution complete. Proof hash: {:016x}", 
             SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
}


fn real_capture() {
    use pcap::Device;
    let device = Device::lookup().unwrap().unwrap();
    let mut cap = device.open().unwrap();
    cap.filter("tcp port 80 or tcp port 443").unwrap();
    while let Ok(packet) = cap.next() {
       
    }
}
