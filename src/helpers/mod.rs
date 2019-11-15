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

pub fn parse_params() -> ArgMatches<'static> {
    App::new("cert-checker")
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
                .help("Time before expiration that should be warning in days")
                .default_value("5")
        )
        .arg(
            Arg::with_name("json")
                .short("j")
                .help("json output log")
            )
        .arg(
            Arg::with_name("domain")
                .required(true)
                .short("d")
                .help("Domain names to check")
                .min_values(1)
        )
        .get_matches()
}