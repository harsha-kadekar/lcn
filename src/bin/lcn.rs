use clap::Parser;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Table};
use lcn::{HostInfo, LCN_PORT};

#[derive(Parser)]
#[command(name = "lcn")]
#[command(about = "Local Computers in Network - Discover LCN hosts on your network")]
#[command(version)]
struct Cli {
    /// Host to query (defaults to localhost)
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    /// Port to connect to
    #[arg(short, long, default_value_t = LCN_PORT)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let url = format!("http://{}:{}/scanhosts", cli.host, cli.port);

    println!("Scanning network for LCN hosts...\n");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120)) // Long timeout for full scan
        .build()
        .expect("Failed to create HTTP client");

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<HostInfo>>().await {
                    Ok(hosts) => display_hosts(&hosts),
                    Err(e) => eprintln!("Failed to parse response: {}", e),
                }
            } else {
                eprintln!("Server returned error: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to LCN server at {}: {}", url, e);
            eprintln!("\nMake sure the LCN server is running:");
            eprintln!("  lcn-server");
        }
    }
}

fn display_hosts(hosts: &[HostInfo]) {
    if hosts.is_empty() {
        println!("No LCN hosts found on the network.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["Hostname", "IPv4 Address"]);

    for host in hosts {
        table.add_row(vec![&host.hostname, &host.hostipv4]);
    }

    println!("{table}");
    println!(
        "\nFound {} host{} running LCN service",
        hosts.len(),
        if hosts.len() == 1 { "" } else { "s" }
    );
}
