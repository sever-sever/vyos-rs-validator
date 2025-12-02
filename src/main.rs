use clap::{Parser, Subcommand};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process;

/// VyOS Rust Validator CLI
#[derive(Parser)]
#[command(name = "vyos-rs-validator")]
#[command(about = "Validate IP addresses", long_about = None)]
struct Cli {
    #[command(subcommand)]
    validator: Validators,
}

#[derive(Subcommand)]
enum Validators {
    /// Validate any IP (IPv4 or IPv6)
    IpHost {
        #[arg(help = "IP address")]
        value: String,
    },
    /// Validate IPv4 address
    Ipv4Host {
        #[arg(help = "IPv4 address")]
        value: String,
    },
    /// Validate IPv6 address
    Ipv6Host {
        #[arg(help = "IPv6 address")]
        value: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let rc = match cli.validator {
        Validators::IpHost { value } => {
            if value.parse::<IpAddr>().is_ok() {
                println!("Valid IP: {}", value);
                0
            } else {
                eprintln!("Invalid IP: {}", value);
                1
            }
        }
        Validators::Ipv4Host { value } => {
            if value.parse::<Ipv4Addr>().is_ok() {
                println!("Valid IPv4: {}", value);
                0
            } else {
                eprintln!("Invalid IPv4: {}", value);
                1
            }
        }
        Validators::Ipv6Host { value } => {
            if value.parse::<Ipv6Addr>().is_ok() {
                println!("Valid IPv6: {}", value);
                0
            } else {
                eprintln!("Invalid IPv6: {}", value);
                1
            }
        }
    };

    process::exit(rc);
}
