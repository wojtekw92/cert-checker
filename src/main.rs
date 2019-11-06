use ssl_expiration::SslExpiration;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};


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
            Arg::with_name("domain")
                .required(true)
                .short("d")
                .help("Domain names to check")
                .min_values(1)
        )
        .get_matches();
    let time: u64 = match matches.value_of("time") {
        Some(val) => {
            match val.parse::<u64>() {
                Ok(val) => val,
                Err(e) => {
                    eprintln! ("Can not parse time value, using default error: {}", e);
                    300
                }
            }
        },
        None => 300
    };

    let domains: Vec<_> = match matches.values_of_lossy("domain") {
        Some(val) => val.clone(),
        None => vec![]
    };

    let handle = thread::spawn(move || loop { //remove later this move
        for domain in &domains {
            let exp = SslExpiration::from_domain_name(&domain);
            match exp {
                Ok(expiration) => {
                    let days_left = expiration.days();
                    if days_left > 5 {
                        println!("Domain \"{}\" is vaild and will be for next {} days!", domain, days_left);
                    } else if days_left > 0 {
                        println!("Domain \"{}\" will be invaild in {} days!", domain, days_left);
                    } else {
                        println!("Error domain \"{}\" is invaild for {} days!", domain, -days_left);
                    }


                },
                Err(e) => println!("Error for domain \"{}\": {}", domain, e)
            }
        }
        thread::sleep(Duration::from_secs(time));
    });

    handle.join().unwrap();

    
}