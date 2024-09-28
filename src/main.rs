use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // KuCoin API'sinden verileri çek
    let resp = reqwest::get("https://api.kucoin.com/api/v1/market/allTickers")
        .await?
        .json::<Value>()
        .await?;

    // USDT çiftlerini filtrele ve formatla
    let mut pairs: Vec<String> = resp["data"]["ticker"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|ticker| ticker["symbol"].as_str().unwrap().ends_with("-USDT"))
        .map(|ticker| {
            let symbol = ticker["symbol"].as_str().unwrap().replace("-USDT", "");
            format!("KUCOIN:{}USDT", symbol)
        })
        .collect();

    // Özel sıralama fonksiyonu
    pairs.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(":").collect();
        let b_parts: Vec<&str> = b.split(":").collect();
        let a_coin = a_parts[1].trim_end_matches("USDT");
        let b_coin = b_parts[1].trim_end_matches("USDT");

        // Sayısal ve alfabetik sıralama
        if a_coin.chars().next().unwrap().is_numeric()
            && b_coin.chars().next().unwrap().is_numeric()
        {
            b_coin.cmp(a_coin)
        } else if a_coin.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Less
        } else if b_coin.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Greater
        } else {
            a_coin.cmp(b_coin)
        }
    });

    // Dosyaya yaz
    let mut file = File::create("kucoin_usdt_markets.txt")?;
    for pair in pairs {
        writeln!(file, "{}", pair)?;
    }

    println!("Veriler başarıyla 'kucoin_usdt_markets.txt' dosyasına yazıldı.");
    Ok(())
}
