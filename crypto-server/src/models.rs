use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AggTrade{
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
     #[serde(rename = "p")]
    pub price: String,
     #[serde(rename = "q")]
    pub qty: String,
     #[serde(rename = "f")]
    pub first_trade_id: u64,
     #[serde(rename = "l")]
    pub last_trade_id: u64,
     #[serde(rename = "T")]
    pub timestamp: i64,
     #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binance_json_aggtrade(){
        let json = r#"{"a":1000,"p":"50000.00","q":"0.1","f":5000,"l":5001,"T":1700000000000,"m":true}"#;
        let t: AggTrade = serde_json::from_str(json).unwrap();

        assert_eq!(t.agg_trade_id, 1000);
        assert_eq!(t.price, "50000.00");
        assert_eq!(t.qty, "0.1");
        assert_eq!(t.timestamp, 1_700_000_000_000);
        assert!(t.is_buyer_maker);
    }
}