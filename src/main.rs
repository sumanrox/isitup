#![allow(warnings)]
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

fn check_domain(domain: &str, hit_counter: &Arc<Mutex<u32>>, fail_counter: &Arc<Mutex<u32>>) {
    let ping_result = Command::new("/bin/ping")
        .arg("-c1")
        .arg("-W1")
        .arg(domain)
        .output();

    match ping_result {
        Ok(output) if output.status.success() => {
            println!("{}", domain);
            let mut hit_counter = hit_counter.lock().unwrap();
            *hit_counter += 1;
        }
        _ => {
            let mut fail_counter = fail_counter.lock().unwrap();
            *fail_counter += 1;
        }
    }
}

fn recon_validate_site(target: &str, num_threads: usize) {
    // Use num_threads within the function to remove the warning
    if let Ok(file) = File::open(target) {
        println!("\n[+] Checking Wordlist");
        println!("----------------------");
        thread::sleep(std::time::Duration::from_secs(2));

        let domains: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        let hit_counter = Arc::new(Mutex::new(0u32));
        let fail_counter = Arc::new(Mutex::new(0u32));

        let mut handles = Vec::new();

        for domain in domains {
            let hit_counter = Arc::clone(&hit_counter);
            let fail_counter = Arc::clone(&fail_counter);

            let handle = thread::spawn(move || {
                check_domain(&domain, &hit_counter, &fail_counter);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let hit_counter = hit_counter.lock().unwrap();
        let fail_counter = fail_counter.lock().unwrap();

        println!("----------------------");
        println!("[+] ALIVE {}", *hit_counter);
        println!("[+] DEAD {}", *fail_counter);
    } else if let Ok(ip) = target.parse::<IpAddr>() {
        println!("\n[+] Checking IP Address");
        println!("------------------------");

        let ping_result = Command::new("/bin/ping")
            .arg("-c1")
            .arg("-W1")
            .arg(ip.to_string())
            .output();

        match ping_result {
            Ok(output) if output.status.success() => {
                println!("[+] Target [{}] is up", ip);
            }
            _ => {
                println!("[-] Target [{}] is down", ip);
            }
        }
    } else {
        println!("Usage [single domain]: isitup domain.tld : Check if a domain is up or not");
        println!("Usage [ip address]   : isitup target.ip : Check if an IP address is up or not");
        println!("Usage [wordlist]     : isitup domain.list : Check alive domains from a domain list");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <target> <num_threads>", args[0]);
        return;
    }
    let target = &args[1];
    let num_threads = match args[2].parse::<usize>() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid number of threads specified.");
        return;
    }
};

recon_validate_site(target, num_threads);
}
