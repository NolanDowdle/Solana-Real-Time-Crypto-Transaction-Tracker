use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use std::str::FromStr;
use reqwest::blocking::get; // For synchronous requests
use serde_json::Value as serdeValue;
use num_format::{Locale, ToFormattedString};
use redis::{Client, Commands}; // Import for synchronous Redis commands

fn fetch_sol_price() -> Result<f64, Box<dyn Error>> {
    let response: serdeValue = reqwest::blocking::get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")?
        .json()?;

    let price = response.get("solana")
                        .and_then(|solana| solana.get("usd"))
                        .and_then(|usd| usd.as_f64())
                        .ok_or("Failed to get Solana price")?;

    Ok(price)
}

fn new_nickname(con: &mut redis::Connection, wallet_address: &str, nickname: &str) -> Result<(), Box<dyn Error>> {
    let key = format!("nickname:{}", wallet_address);
    con.set(key, nickname)?;
    Ok(())
}

fn get_nickname(con: &mut redis::Connection, wallet_address: &str) -> Result<String, Box<dyn Error>> {
    let key = format!("nickname:{}", wallet_address);
    let nickname: String = con.get(key)?;
    Ok(nickname)
}

fn main() -> Result<(), Box<dyn Error>> {
    let solana_url = "https://api.mainnet-beta.solana.com"; // or another RPC URL
    let client = RpcClient::new(solana_url.to_string());

    // Connect to AWS ElastiCache Redis
    let redis_url = "redis://127.0.0.1/"; // Use "rediss" for TLS
    let redis_client = Client::open(redis_url)?; // Create a Redis client
    let mut con = match redis_client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            return Err(Box::new(e)); // Or handle it as needed
        },
    };

    let addresses = vec![
        Pubkey::from_str("9UHmNKShXzLUzQDkZikzeVAVFeMQxi9E9M8pn7myR48f")?, // phantom 1
        Pubkey::from_str("CuieVDEDtLo7FypA9SbLM9saXFdb1dsshEkyErMqkRQq")?, // famous tracked wallet
        Pubkey::from_str("8BseXT9EtoEhBTKFFYkwTnjKSUZwhtmdKY2Jrj8j45Rt")?, // millionaire wallet
        Pubkey::from_str("4sWu7gsYuccocqRj3uVQGYudJukArbGrQtR3JG7yLXdv")?, // cb wallet
        Pubkey::from_str("7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy")?, // honeyland
    ];

    new_nickname(&mut con, "7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy", "Honey Land")?; 
    let mut nickname = get_nickname(&mut con, "7BjDo2QxCev6qF98R8dvj5u7WRVf8RNDgiq5PUDL65Yy")?;

    // Fetch account balance
    let sol_price = fetch_sol_price()?; // Call synchronously
    println!("Current Solana price: ${}", sol_price);

    for pubkey in addresses {
        let balance = client.get_balance(&pubkey)?;
        let balance_sol = balance as f64 / 1_000_000_000.0; // Convert lamports to SOL
    
        // Get nickname for wallet, which is a Result
        let nickname_result = get_nickname(&mut con, &pubkey.to_string());
    
        match nickname_result {
            Ok(nick) => {
                // If nickname is found, print it
                println!("SOL Account balance for {}, with nickname \"{}\": {}", pubkey, nick, balance_sol);
            },
            Err(e) => {
                // If there is an error (e.g., no nickname found)
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
