use clap::Parser;
mod multiple;
mod single;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Checks a single website
    #[clap(short = 's', long = "single")]
    site: Option<String>,
    /// Checks multiple websites (space separated)
    #[clap(short = 'm', long = "multiple", value_delimiter = ',')]
    sites: Option<Vec<String>>,
    /// Adds more details to the responses
    #[clap(short = 'd', long = "detail", action)]
    detail: bool,
}

fn main() {
    let args = Args::parse();

    assert_ne!(
        (args.site != None) || (args.sites != None),
        false,
        "\nError: Specify at least one flag and one argument.\n\nFor more information try '--help'\n"
    );
    print!("\n");

    let mut detailed = false;
    if args.detail {
        detailed = true;
        let host = dns_lookup::get_hostname();
        println!("Host: {}\n", host.unwrap().to_string());
    }
    if args.site != None {
        single::run(args.site.clone().unwrap(), detailed);
    }
    if args.sites != None {
        multiple::run(args.sites.clone().unwrap(), detailed);
    }
}
