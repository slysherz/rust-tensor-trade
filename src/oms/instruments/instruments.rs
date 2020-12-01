
#[derive(PartialEq, Eq, Debug)]
pub struct Instrument {
    pub symbol: String,
    pub precision: u32,
    pub name: String
}

impl Instrument {
    pub fn new(symbol: &str, precision: u32, name: &str) -> Instrument {
        Instrument {
            symbol: symbol.to_string(),
            precision,
            name: name.to_string()
        }
    }

    pub fn clone(&self) -> Instrument {
        Instrument {
            symbol: self.symbol.clone(),
            precision: self.precision,
            name: self.name.clone()
        }
    }

    // Crypto
    pub fn btc() -> Instrument {
        Instrument{ symbol: "BTC".to_string(), precision: 8, name: "Bitcoin".to_string() }
    } 
    pub fn eth() -> Instrument {
        Instrument{ symbol: "ETH".to_string(), precision: 8, name: "Ethereum".to_string() }
    } 
    pub fn xrp() -> Instrument {
        Instrument{ symbol: "XRP".to_string(), precision: 8, name: "XRP".to_string() }
    } 
    pub fn neo() -> Instrument {
        Instrument{ symbol: "NEO".to_string(), precision: 8, name: "NEO".to_string() }
    } 
    pub fn bch() -> Instrument {
        Instrument{ symbol: "BCH".to_string(), precision: 8, name: "Bitcoin Cash".to_string() }
    } 
    pub fn ltc() -> Instrument {
        Instrument{ symbol: "LTC".to_string(), precision: 8, name: "Litecoin".to_string() }
    } 
    pub fn etc() -> Instrument {
        Instrument{ symbol: "ETC".to_string(), precision: 8, name: "Ethereum Classic".to_string() }
    } 
    pub fn xlm() -> Instrument {
        Instrument{ symbol: "XLM".to_string(), precision: 8, name: "Stellar Lumens".to_string() }
    } 
    pub fn link() -> Instrument {
        Instrument{ symbol: "LINK".to_string(), precision: 8, name: "Chainlink".to_string() }
    } 
    pub fn atom() -> Instrument {
        Instrument{ symbol: "ATOM".to_string(), precision: 8, name: "Cosmos".to_string() }
    } 
    pub fn dai() -> Instrument {
        Instrument{ symbol: "DAI".to_string(), precision: 8, name: "Dai".to_string() }
    } 
    pub fn usdt() -> Instrument {
        Instrument{ symbol: "USDT".to_string(), precision: 8, name: "Tether".to_string() }
    } 
    
    // FX
    pub fn usd() -> Instrument {
        Instrument{ symbol: "USD".to_string(), precision: 2, name: "U.S. Dollar".to_string() }
    } 
    pub fn eur() -> Instrument {
        Instrument{ symbol: "EUR".to_string(), precision: 2, name: "Euro".to_string() }
    } 
    pub fn jpy() -> Instrument {
        Instrument{ symbol: "JPY".to_string(), precision: 2, name: "Japanese Yen".to_string() }
    } 
    pub fn kwn() -> Instrument {
        Instrument{ symbol: "KWN".to_string(), precision: 2, name: "Korean Won".to_string() }
    } 
    pub fn aud() -> Instrument {
        Instrument{ symbol: "AUD".to_string(), precision: 2, name: "Australian Dollar".to_string() }
    } 
    
    // Commodities
    pub fn xau() -> Instrument {
        Instrument{ symbol: "XAU".to_string(), precision: 2, name: "Gold futures".to_string() }
    } 
    pub fn xag() -> Instrument {
        Instrument{ symbol: "XAG".to_string(), precision: 2, name: "Silver futures".to_string() }
    } 
    
    // Stocks
    pub fn aapl() -> Instrument {
        Instrument{ symbol: "AAPL".to_string(), precision: 2, name: "Apple stock".to_string() }
    } 
    pub fn msft() -> Instrument {
        Instrument{ symbol: "MSFT".to_string(), precision: 2, name: "Microsoft stock".to_string() }
    } 
    pub fn tsla() -> Instrument {
        Instrument{ symbol: "TSLA".to_string(), precision: 2, name: "Tesla stock".to_string() }
    } 
    pub fn amzn() -> Instrument {
        Instrument{ symbol: "AMZN".to_string(), precision: 2, name: "Amazon stock".to_string() }
    } 
}
