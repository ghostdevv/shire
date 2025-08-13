use clap::Parser;
use color_eyre::eyre::Result;

mod cloudflare;
mod ip;
mod records;
mod utils;

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

    #[arg(long, short, env = "CLOUDFLARE_API_TOKEN", help = "Your Cloudflare API token")]
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

    println!("Checking records...");
    let mut updates_body = records::UpdateRecordsBodyBuilder::new(ip.to_owned());

    for record_name in args.records {
        let record = records.get(&record_name);

        match record {
            None => {
                println!("  Creating: \"{}\" with ip \"{}\"", record_name, ip);
                updates_body.create(record_name);
            }
            Some(record) => {
                println!("  Updating: \"{}\" with ip \"{}\"", record_name, ip);
                updates_body.update(
                    record.id.to_owned(),
                    record.name.to_owned(),
                    record.comment.to_owned(),
                );
            }
        }
    }

    println!("Saving changes...");
    records::update_records(&args.zone_id, &args.key, &updates_body).await?;

    println!("Done!");
    Ok(())
}
