use clap::Parser;
use yahoo_finance_api::YahooConnector;

#[derive(Parser)]
struct Cli {
	ticker: String,
}

#[tokio::main]
async fn main() {
	let args = Cli::parse();

	let yahoo = match YahooConnector::new() {
		Ok(connector) => connector,
		Err(e) => {
			println!("Failed to initialize YahooConnector: {}", e);
			return;
		}
	};

	match yahoo.get_latest_quotes(&args.ticker, "1d").await {
		Ok(response) => {
			let quote = response.last_quote().unwrap();

			println!("Stock info for: ${}", args.ticker);
			println!("Price: ${}", quote.close);
			println!("High: ${}", quote.high);
			println!("Low: ${}", quote.low);
			println!("Volume: ${}", quote.volume);
		}
		Err(e) => {
			println!("Error: {}", e);
		}
	}
}

