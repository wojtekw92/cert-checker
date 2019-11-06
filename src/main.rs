use ssl_expiration::SslExpiration;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};

use serde::{Serialize, Deserialize};
use serde_json;
use std::str::FromStr;
use chrono::Local;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
enum CertyficateStatus {
    Valid,
    SoonInvalid,
    Invalid,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct CertyficateData {
    domain: String,
    status: CertyficateStatus,
    time_stamp: String,
    #[serde(skip_serializing_if="Option::is_none")]
    expire_in: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    expired_for: Option<i32>,
}

fn get_value<T: FromStr>(input: Option<&str>, default_value: T ) -> T {
    match input {
        Some(val) => {
            match val.parse::<T>() {
                Ok(val) => val,
                Err(_e) => {
                    eprintln! ("Can not parse value, using `get_value`!");
                    default_value
                }
            }
        },
        None => default_value
    }
}

fn main() {

    let matches = App::new("cert-checker")
        .version("1.0")
        .author("Wojciech Wozniak <wwozniak92@gmail.com>")
        .about("certyficate checking tool")
        .arg(
            Arg::with_name("time")
                .short("t")
                .help("The sleep time between tests")
                .default_value("300")
        )
        .arg(
            Arg::with_name("left")
                .short("l")
                .help("The day before expiration that should be warning")
                .default_value("5")
        )
        .arg(
            Arg::with_name("domain")
                .required(true)
                .short("d")
                .help("Domain names to check")
                .min_values(1)
        )
        .get_matches();
    let time: u64 = get_value(matches.value_of("time"), 300);
    let left_warning_time: i32 = get_value(matches.value_of("left"), 5);
    

    let domains: Vec<_> = match matches.values_of_lossy("domain") {
        Some(val) => val.clone(),
        None => vec![]
    };

    let handle = thread::spawn(move || loop { //remove later this "move" for now we Hacky :) 
        for domain in &domains {
            let exp = SslExpiration::from_domain_name(&domain);
            match exp {
                Ok(expiration) => {
                    let status = CertyficateData {
                        domain: domain.to_string(),
                        time_stamp: Local::now().to_rfc3339(),
                        status: match expiration.days() {
                            x if x > left_warning_time => CertyficateStatus::Valid,
                            x if x > 0  => CertyficateStatus::SoonInvalid,
                            _ => CertyficateStatus::Invalid
                        },
                        expire_in: match expiration.days() {
                            x if x>0 => Some(x),
                            _ => None
                        },
                        expired_for: match expiration.days() {
                            x if x<=0 => Some(-x),
                            _ => None
                        }
                    };
                    match serde_json::to_string(&status) {
                        Ok(x) => println!("{}", x),
                        Err(e) => eprintln!("Error during parsing CertyficateData: {}", e)
                    }
                    
                },
                Err(e) => println!("Error for domain \"{}\": {}", domain, e)
            }
        }
        thread::sleep(Duration::from_secs(time));
    });

    handle.join().unwrap();

    
}
