pub mod crypto_data_download {
    use csv::StringRecord;
    use serde::Deserialize;
    use std::io::{BufRead, BufReader};

    #[derive(Deserialize, Debug)]
    pub struct DefaultRecord {
        pub unix_timestamp: String,
        pub date: String,
        pub open: f32,
        pub high: f32,
        pub low: f32,
        pub close: f32,
        pub volume: f32,
    }

    #[derive(Deserialize, Debug)]
    pub struct DefaultRecordAll {
        pub unix_timestamp: String,
        pub date: String,
        pub open: f32,
        pub high: f32,
        pub low: f32,
        pub close: f32,
        pub volume_quote: f32,
        pub volume_base: f32,
    }

    #[derive(Deserialize, Debug)]
    pub struct GeminiRecord {
        pub date: String,
        pub open: f32,
        pub high: f32,
        pub low: f32,
        pub close: f32,
        pub volume: f32,
    }

    fn rename_column(headers: StringRecord, from: String, to: String) -> StringRecord {
        let mut new_headers = StringRecord::new();

        for header in headers.iter() {
            let field = if header == from { to.as_str() } else { header };

            new_headers.push_field(field);
        }

        new_headers
    }

    fn lowcase(headers: StringRecord) -> StringRecord {
        let mut new_headers = StringRecord::new();

        for header in headers.iter() {
            new_headers.push_field(header.to_lowercase().as_str());
        }

        new_headers
    }

    static URL: &str = "https://www.cryptodatadownload.com/cdd/";

    pub fn fetch_default(
        exchange_name: String,
        base_symbol: String,
        quote_symbol: String,
        timeframe: String,
    ) -> Result<Vec<DefaultRecord>, Box<dyn std::error::Error>> {
        let filename = format!(
            "{}_{}{}_{}.csv",
            exchange_name, quote_symbol, base_symbol, timeframe
        );
        let timestamp = "Unix Timestamp".to_string();
        let new_timestamp = "unix_timestamp".to_string();
        let base_vc = format!("Volume {}", base_symbol);
        let new_base_vc = "volume".to_string();

        // Creates a new csv `Reader` from a file
        let mut url: String = URL.to_owned();
        url.push_str(filename.as_str());

        let resp = reqwest::blocking::get(url.as_str())?.text()?;
        let mut data = BufReader::new(resp.as_bytes());

        // skip the first line
        data.read_until(b'\n', &mut Vec::new())?;

        let mut reader = csv::Reader::from_reader(data);

        // change header names to match our needs
        let headers = reader.headers()?.clone();
        let headers = rename_column(headers, timestamp, new_timestamp);
        let headers = rename_column(headers, base_vc, new_base_vc);

        reader.set_headers(lowcase(headers));

        let mut df = Vec::new();
        for result in reader.deserialize() {
            let record: DefaultRecord = result?;
            df.push(record);
        }

        Ok(df)
    }

    pub fn fetch_default_all(
        exchange_name: String,
        base_symbol: String,
        quote_symbol: String,
        timeframe: String,
    ) -> Result<Vec<DefaultRecordAll>, Box<dyn std::error::Error>> {
        let filename = format!(
            "{}_{}{}_{}.csv",
            exchange_name, quote_symbol, base_symbol, timeframe
        );
        let timestamp = "Unix Timestamp".to_string();
        let new_timestamp = "unix_timestamp".to_string();
        let base_vc = format!("Volume {}", base_symbol);
        let new_base_vc = "volume_base".to_string();
        let quote_vc = format!("Volume {}", quote_symbol);
        let new_quote = "volume_quote".to_string();

        // Creates a new csv `Reader` from a file
        let mut url: String = URL.to_owned();
        url.push_str(filename.as_str());

        let resp = reqwest::blocking::get(url.as_str())?.text()?;
        let mut data = BufReader::new(resp.as_bytes());

        // skip the first line
        data.read_until(b'\n', &mut Vec::new())?;

        let mut reader = csv::Reader::from_reader(data);

        // change header names to match our needs
        let headers = reader.headers()?.clone();
        let headers = rename_column(headers, timestamp, new_timestamp);
        let headers = rename_column(headers, base_vc, new_base_vc);
        let headers = rename_column(headers, quote_vc, new_quote);

        reader.set_headers(lowcase(headers));

        let mut df = Vec::new();
        for result in reader.deserialize() {
            let record: DefaultRecordAll = result?;
            df.push(record);
        }

        Ok(df)
    }

    pub fn fetch_gemini(
        base_symbol: String,
        quote_symbol: String,
        timeframe: String,
    ) -> Result<Vec<GeminiRecord>, Box<dyn std::error::Error>> {
        let timeframe = if timeframe.ends_with("h") {
            timeframe + "r"
        } else {
            timeframe
        };

        let filename = format!(
            "{}_{}{}_{}.csv",
            "gemini", quote_symbol, base_symbol, timeframe
        );

        // Creates a new csv `Reader` from a file
        let mut url: String = URL.to_owned();
        url.push_str(filename.as_str());

        let resp = reqwest::blocking::get(url.as_str())?.text()?;
        let mut data = BufReader::new(resp.as_bytes());

        // skip the first line
        data.read_until(b'\n', &mut Vec::new())?;

        let mut reader = csv::Reader::from_reader(data);

        // change header names to match our needs
        let headers = reader.headers()?.clone();

        reader.set_headers(lowcase(headers));

        let mut df = Vec::new();
        for result in reader.deserialize() {
            let record: GeminiRecord = result?;
            df.push(record);
        }

        Ok(df)
    }
}
