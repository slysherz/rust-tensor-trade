use rust_tensortrade::data::crypto_data_download;

#[test]
fn test_crypto_download() {
    println!("\nCOINBASE DEFAULT");
    for e in crypto_data_download::fetch_default(
        "Coinbase".to_string(),
        "USD".to_string(),
        "BTC".to_string(),
        "1h".to_string(),
    )
    .unwrap()
    .iter()
    .take(5)
    {
        println!("{:?}", e);
    }

    println!("\nCOINBASE ALL");
    for e in crypto_data_download::fetch_default_all(
        "Coinbase".to_string(),
        "USD".to_string(),
        "BTC".to_string(),
        "1h".to_string(),
    )
    .unwrap()
    .iter()
    .take(5)
    {
        println!("{:?}", e);
    }

    println!("\nGEMINI");
    for e in
        crypto_data_download::fetch_gemini("USD".to_string(), "BTC".to_string(), "1h".to_string())
            .unwrap()
            .iter()
            .take(5)
    {
        println!("{:?}", e);
    }
}
