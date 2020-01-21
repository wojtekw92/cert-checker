use ssl_expiration::SslExpiration;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use serde_json;
// use std::collections::HashMap;
use opsgenie::*;

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
    // let webhook_url = match &matches.value_of_lossy("webhook") {
    //     Some(x) => Some(x.to_string()),
    //     None => None
    // };
    let opsgenie_key = match &matches.value_of_lossy("opsgenie") {
        Some(x) => Some(x.to_string()),
        None => None
    };
    let (tx, rx) = mpsc::channel();

    let _domain_checking_thread = thread::spawn(move || loop { //remove later this "move" for now we are Hacky :)
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
                        }
                    );
                    match matches.is_present("json") {
                        true => match serde_json::to_string(&status) {
                            Ok(x) => println!("{}", x),
                            Err(e) => eprintln!("Error during parsing CertyficateData: {}", e)
                        },
                        false => println!("[{}] {}  status: {:?} left: {}",
                            status.time_stamp,
                            status.domain,
                            status.status,
                            match status.expire_in {
                                Some(x) =>x,
                                None => 0,
                            }
                        )
                    };

                    // Add opsgenie integration below instead logging
                    // match status.status {
                    //     CertyficateStatus::Valid => (),
                    //     CertyficateStatus::SoonInvalid => println!("Warning! Certyficate for domain {}  will expire in {} days!", status.domain, status.expire_in.unwrap()),
                    //     CertyficateStatus::Invalid => println!("Error Certyficate for domain {} is invalid!", status.domain)
                    // }
                    tx.send(status).unwrap()
                },
                Err(e) => println!("Error for domain \"{}\": {}", domain, e)
            }
        }
        thread::sleep(Duration::from_secs(time));
    });

    // domain_checking_thread.join().unwrap();
    for received in rx {
        // println!("Got: {:?}", received.status);

        if received.status == CertyficateStatus::Valid {

        } else if received.status == CertyficateStatus::SoonInvalid {
            if let Some(key) = &opsgenie_key {
                let opsgenie = OpsGenie::new(key.to_string());
                let alert = AlertData::new(format!("Warning! Certyficate for domain {}  will expire in {} days!", received.domain, received.expire_in.unwrap()).to_string())
                    .alias(format!("{} certyficate issues!", received.domain).to_string())
                    .source("cert-checker".to_string())
                    .entity(received.domain)
                    .priority(Priority::P5);
                match opsgenie.alert(alert) {
                    Err(e) => println!("Error while sending alert with msg {}!", e),
                    _ => (),
                }
            }

        } else {
            if let Some(key) = &opsgenie_key {
                let opsgenie = OpsGenie::new(key.to_string());
                let alert = AlertData::new(format!("Error Certyficate for domain {} is invalid!", received.domain).to_string())
                    .alias(format!("{} certyficate issues!", received.domain).to_string())
                    .source("cert-checker".to_string())
                    .entity(received.domain)
                    .priority(Priority::P1);
                match opsgenie.alert(alert) {
                    Err(e) => println!("Error while sending alert with msg {}!", e),
                    _ => (),
                }
            }
        }
        // if let Some(url) = &webhook_url {
        //     println!("Will send request here: {}", url)
        // }
    }
}
