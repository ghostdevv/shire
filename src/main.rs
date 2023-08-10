use clap::Parser;

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

    #[arg(long, short, help = "The Cloudflare Zone ID for your domain")]
    zone_id: String,

    #[arg(long, short, help = "Your Cloudflare API key")]
    key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let records = records::get_records(&args.zone_id, &args.key)
        .await
        .expect("Failed to get records");

    let ip = ip::get_ip().await.expect("Failed to get IP");

    for record_name in args.records {
        let record_id = records
            .get(&record_name)
            .expect(&format!("Unable to find record '{}'", record_name));

        records::set_ip(&args.zone_id, &record_id, &ip, &args.key)
            .await
            .expect("Failed to set IP");

        println!("Updating record '{}'", record_name);
    }

    println!("{:?}", records);
}
