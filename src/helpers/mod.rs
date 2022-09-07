use std::str::FromStr;

use clap::{App, Arg, ArgMatches};


pub fn get_value<T: FromStr>(input: Option<&str>, default_value: T ) -> T {
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

pub fn parse_params() -> ArgMatches {
    App::new("cert-checker")
        .version("1.0")
        .author("Wojciech Wozniak <wwozniak92@gmail.com>")
        .about("certyficate checking tool")
        .arg(
            Arg::with_name("time")
                .short('t')
                .help("The sleep time between tests")
                .default_value("300")
        )
        .arg(
            Arg::with_name("left")
                .short('l')
                .help("Time before expiration that should be warning in days")
                .default_value("5")
        )
        .arg(
            Arg::with_name("webhook")
                .short('w')
                .help("Webhook for failed or failing soon certs")
                .allow_invalid_utf8(true)
                .value_name("URL")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("opsgenie")
                .short('o')
                .allow_invalid_utf8(true)
                .help("opsgenie intergration to infrom about soon failing certs")
                .value_name("API-KEY")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("json")
                .short('j')
                .help("json output log")
            )
        .arg(
            Arg::with_name("domain")
                .required(true)
                .short('d')
                .allow_invalid_utf8(true)
                .help("Domain names to check")
                .min_values(1)
        )
        .get_matches()
}