use serde::Deserialize;
use ureq::serde_json;
use std::{fs::File, io::Write, thread, time::Duration};

// Struct Definitions
#[derive(Deserialize, Debug)]
struct Bitcoin {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct Ethereum {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct SP500 {
    price: f64,
}

// Trait Definition
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self, filename: &str) -> std::io::Result<()>;
}

// Implementing Pricing for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value =
            ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").call()?.into_json()?;
        self.price = response["bitcoin"]["usd"].as_f64().unwrap();
        Ok(())
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(filename)?;
        write!(file, "Bitcoin Price: {:.2}\n", self.price)
    }
}

// Implementing Pricing for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value =
            ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd").call()?.into_json()?;
        self.price = response["ethereum"]["usd"].as_f64().unwrap();
        Ok(())
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(filename)?;
        write!(file, "Ethereum Price: {:.2}\n", self.price)
    }
}

// Implementing Pricing for SP500
impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d")
            .call()?
            .into_json()?;
        
        // Navigate through the JSON structure
        let results = response["chart"]["result"]
            .as_array()
            .ok_or("Missing 'result' array in response")?;
        
        let indicators = &results[0]["indicators"]["quote"]
            .as_array()
            .ok_or("Missing 'quote' array in response")?;
        
        let close_prices = &indicators[0]["close"]
            .as_array()
            .ok_or("Missing 'close' array in response")?;
        
        // Get the most recent close price
        self.price = close_prices
            .last()
            .ok_or("No closing prices available")?
            .as_f64()
            .ok_or("Invalid closing price format")?;
        
        Ok(())
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(filename)?;
        write!(file, "S&P 500 Price: {:.2}\n", self.price)
    }
}

// Main Function
fn main() {
    let mut bitcoin = Bitcoin { price: 0.0 };
    let mut ethereum = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    let mut assets: Vec<(&mut dyn Pricing, &str)> = vec![
        (&mut bitcoin, "bitcoin_pricing_data.txt"), 
        (&mut ethereum, "ethereum_pricing_data.txt"), 
        (&mut sp500, "sp500_pricing_data.txt")
    ];

    loop {
        for (asset, filename) in &mut assets {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Error fetching price: {}", e);
                continue;
            }
            if let Err(e) = asset.save_to_file(filename) {
                eprintln!("Error saving to file: {}", e);
            }
        }
        println!("Prices updated. Waiting for 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}

