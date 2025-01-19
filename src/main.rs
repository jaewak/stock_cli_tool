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
			println!("Stock info ${}", response.last_quote().unwrap().close);
		}
		Err(e) => {
			println!("Error: {}", e);
		}
	}
}
