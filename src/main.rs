use ssl_expiration::SslExpiration;
use std::thread;
use std::time::Duration;
use serde_json;

mod helpers;
mod cert_info;
use cert_info::{CertyficateStatus, CertyficateData};

fn main() {
    let matches = helpers::parse_params();
    let time: u64 = helpers::get_value(matches.value_of("time"), 300);
    let left_warning_time: i32 = helpers::get_value(matches.value_of("left"), 5);
    

    let domains: Vec<_> = match matches.values_of_lossy("domain") {
        Some(val) => val.clone(),
        None => vec![]
    };

    let handle = thread::spawn(move || loop { //remove later this "move" for now we Hacky :) 
        for domain in &domains {
            let exp = SslExpiration::from_domain_name(&domain);
            match exp {
                Ok(expiration) => {
                    let status = CertyficateData::new(
                        domain, 
                        expiration.days(),
                        match expiration.days() {
                            x if x > left_warning_time => CertyficateStatus::Valid,
                            x if x > 0  => CertyficateStatus::SoonInvalid,
                            _ => CertyficateStatus::Invalid
                        });

                    match serde_json::to_string(&status) {
                        Ok(x) => println!("{}", x),
                        Err(e) => eprintln!("Error during parsing CertyficateData: {}", e)
                    }
                    // Add opsgenie integration below instead logging
                    match status.status {
                        CertyficateStatus::Valid => (),
                        CertyficateStatus::SoonInvalid => println!("Warning! Certyficate for domain {}  will expire in {} days!", status.domain, status.expire_in.unwrap()),
                        CertyficateStatus::Invalid => println!("Error Certyficate for domain {} is invalid!", status.domain)
                    }
                },
                Err(e) => println!("Error for domain \"{}\": {}", domain, e)
            }
        }
        thread::sleep(Duration::from_secs(time));
    });

    handle.join().unwrap();

    
}
