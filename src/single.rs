use colored::Colorize;
use dns_lookup::lookup_host;
use reqwest::{blocking, Error};
use std::{panic, time::SystemTime};

pub fn run(site: String, detailed: bool) {
    let start = SystemTime::now();

    let call = || -> Result<blocking::Response, Error> {
        let body = blocking::get(site.clone())?;
        Ok(body)
    };

    let res = call();

    let duration = SystemTime::now()
        .duration_since(start)
        .expect("Duration should be counted from 'start'");

    critical_section::with(|_cs| {
        if let Err(_err) = res {
            println!("For site '{}',\n{}", site, _err.to_string().bright_yellow());
        } else {
            let status = res.unwrap().status().to_string();
            print_status(status, site.clone());
            if detailed {
                print_details(site);
            }
            println!("completed in {}s", duration.as_secs_f32());
        }
        println!("------------------");
    })
}

fn print_status(status: String, site: String) {
    let mut iter = status.split_whitespace();
    let code: i16 = iter.next().unwrap().parse::<i16>().unwrap();

    if code >= 100 && code < 200 {
        println!("Site '{}' pinged,\nstatus: {},", site, status.bright_blue());
    } else if code >= 200 && code < 400 {
        println!(
            "Site '{}' pinged,\nstatus: {},",
            site,
            status.bright_green()
        );
    } else {
        println!("Site '{}' pinged,\nstatus: {},", site, status.bright_red());
    }
}

fn print_details(site: String) {
    let hostname = &site.as_str()[8..];

    panic::set_hook(Box::new(|_info| {}));
    let panics = panic::catch_unwind(|| {
        let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
        println!("IPv4: {},", ips[0].to_string());
        return;
    })
    .is_err();

    if panics {
        println!("IPv4: NA,");
    }
}
