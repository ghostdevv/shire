use clap::Parser;
use color_eyre::eyre::Result;

mod cloudflare;
mod ip;
mod records;

#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Args {
    #[arg(
        long,
        short,
        help = "Comma seperated list of the record names to update"
    )]
    records: Vec<String>,

    #[arg(long, short, help = "The Cloudflare Zone Id for your domain")]
    zone_id: String,

    #[arg(long, short, help = "Your Cloudflare API key")]
    key: String,

    #[arg(
        long,
        short,
        default_value = "https://ip.willow.sh",
        help = "The IP resolver url to use"
    )]
    ip_resolver: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.records.is_empty() {
        println!("No records specified, exiting...");
        return Ok(());
    }

    println!("Fetching record data...");
    let records = records::get_records(&args.zone_id, &args.key).await?;

    println!("Resolving IPv4 address...");
    let ip = ip::get_ip(&args.ip_resolver).await?;

    println!("Updating records...");
    for record_name in args.records {
        let record_id = records
            .get(&record_name)
            .expect(&format!("Unable to find record \"{}\"", record_name));

        println!("  Updating \"{}\" with ip \"{}\"", record_name, ip);
        records::set_ip(&args.zone_id, &record_id, &ip, &args.key).await?;
    }

    println!("Done!");
    Ok(())
}
