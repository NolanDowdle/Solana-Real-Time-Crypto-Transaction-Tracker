use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use std::str::FromStr;
use serde_json::Value as serdeValue;
use num_format::{Locale, ToFormattedString};
use redis::cluster::ClusterClient; // Correctly use redis::cluster::ClusterClient
use redis::Commands; // Import Commands for Redis operations

fn fetch_sol_price() -> Result<f64, Box<dyn Error>> {
    let response: serdeValue = reqwest::blocking::get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")?
        .json()?;

    let price = response.get("solana")
                        .and_then(|solana| solana.get("usd"))
                        .and_then(|usd| usd.as_f64())
                        .ok_or("Failed to get Solana price")?;

    Ok(price)
}

fn new_nickname(client: &ClusterClient, wallet_address: &str, nickname: &str) -> Result<(), Box<dyn Error>> {
    let key = format!("nickname:{}", wallet_address);
    let mut con = client.get_connection()?; // Get a connection for the operation
    con.set(key, nickname)?;
    Ok(())
}

fn get_nickname(client: &ClusterClient, wallet_address: &str) -> Result<String, Box<dyn Error>> {
    let key = format!("nickname:{}", wallet_address);
    let mut con = client.get_connection()?; // Get a connection for the operation
    let nickname: String = con.get(key)?;
    Ok(nickname)
}

fn main() -> Result<(), Box<dyn Error>> {
    let solana_url = "https://api.mainnet-beta.solana.com"; // or another RPC URL
    let client = RpcClient::new(solana_url.to_string());

    // Connect to Redis Cluster
    let redis_urls = vec![
        "redis://127.0.0.1:7001",
        "redis://127.0.0.1:7002",
        "redis://127.0.0.1:7006",
    ];

    // Create a Redis Cluster client
    let cluster_client = ClusterClient::new(redis_urls)?;

    let addresses = vec![
        Pubkey::from_str("9UHmNKShXzLUzQDkZikzeVAVFeMQxi9E9M8pn7myR48f")?, // phantom 1
        Pubkey::from_str("CuieVDEDtLo7FypA9SbLM9saXFdb1dsshEkyErMqkRQq")?, // famous tracked wallet
        Pubkey::from_str("8BseXT9EtoEhBTKFFYkwTnjKSUZwhtmdKY2Jrj8j45Rt")?, // millionaire wallet
        Pubkey::from_str("4sWu7gsYuccocqRj3uVQGYudJukArbGrQtR3JG7yLXdv")?, // cb wallet
        Pubkey::from_str("7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy")?, // honeyland
    ];

    // Set and get nickname example
    new_nickname(&cluster_client, "7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy", "Honey Land")?;
    //let nickname = get_nickname(&cluster_client, "7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy")?;

    // Fetch account balance and print information
    let sol_price = fetch_sol_price()?; // Get SOL price
    println!("Current Solana price: ${}", sol_price);

    for pubkey in addresses {
        let balance = client.get_balance(&pubkey)?;
        let balance_sol = balance as f64 / 1_000_000_000.0; // Convert lamports to SOL

        // Get nickname for wallet
        let nickname_result = get_nickname(&cluster_client, &pubkey.to_string());

        match nickname_result {
            Ok(nick) => {
                // If nickname is found, print it
                println!("SOL Account balance for {}, with nickname \"{}\": {}", pubkey, nick, balance_sol);
            },
            Err(_) => {
                // If no nickname found
                println!("SOL Account balance for {}: No nickname found", pubkey);
            }
        }

        let total_value_usd = sol_price * balance_sol;
        let whole_part = total_value_usd.trunc() as i64;
        let decimal_part = (total_value_usd.fract() * 100.0).round() as i64;

        let formatted_whole = whole_part.to_formatted_string(&Locale::en);
        let formatted_decimal = format!("{:02}", decimal_part);
        let formatted_value = format!("{}.{}", formatted_whole, formatted_decimal);

        println!("Account balance in USD for {}: ${}\n", pubkey, formatted_value);
    }

    Ok(())
}
