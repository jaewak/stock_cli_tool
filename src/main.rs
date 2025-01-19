use yahoo_finance_api::YahooConnector;

#[tokio::main]
async fn main() {
	let yahoo = match YahooConnector::new() {
		Ok(connector) => connector,
		Err(e) => {
			println!("Failed to initialize YahooConnector: {}", e);
			return;
		}
	};

	match yahoo.get_latest_quotes("AAPL", "1d").await {
		Ok(response) => {
			let quote = response.last_quote().unwrap();

			println!("Stock info for: ${}", quote.close);
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

