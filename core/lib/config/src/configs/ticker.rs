// Built-in uses
// External uses
use serde::Deserialize;
// Workspace uses
use zksync_types::Address;
// Local uses
use crate::envy_load;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum TokenPriceSource {
    CoinGecko,
    CoinMarketCap,
}

/// Configuration for the fee ticker.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TickerConfig {
    /// Indicator of the API to be used for getting token prices.
    pub token_price_source: TokenPriceSource,
    /// URL of CoinMarketCap API. Can be set to the mock server for local development.
    pub coinmarketcap_base_url: String,
    /// URL of CoinGecko API. Can be set to the mock server for local development.
    pub coingecko_base_url: String,
    /// Coefficient for the fee price for fast withdrawal requests.
    pub fast_processing_coeff: f64,
    /// Url to uniswap api
    pub uniswap_url: String,
    /// The volume of tokens to confirm their liquidity
    pub liquidity_volume: f64,
    /// Time when liquidity check results are valid
    pub available_liquidity_seconds: u64,
    /// List of the tokens that are unconditionally acceptable for paying fee in.
    pub unconditionally_valid_tokens: Vec<Address>,
    ///
    pub token_market_update_time: u64,
    /// Number of tickers for load balancing.
    pub number_of_ticker_actors: u8,
    /// List of tokens for which subsidions are disabled.
    pub not_subsidized_tokens: Vec<Address>,
}

impl TickerConfig {
    pub fn from_env() -> Self {
        envy_load!("fee_ticker", "FEE_TICKER_")
    }

    /// Returns the token price source type and the corresponding API URL.
    pub fn price_source(&self) -> (TokenPriceSource, &str) {
        let url = match self.token_price_source {
            TokenPriceSource::CoinGecko => self.coingecko_base_url.as_ref(),
            TokenPriceSource::CoinMarketCap => self.coinmarketcap_base_url.as_ref(),
        };

        (self.token_price_source, url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::test_utils::{addr, set_env};

    fn expected_config() -> TickerConfig {
        TickerConfig {
            token_price_source: TokenPriceSource::CoinGecko,
            coinmarketcap_base_url: "http://127.0.0.1:9876".into(),
            coingecko_base_url: "http://127.0.0.1:9876".into(),
            fast_processing_coeff: 10.0f64,
            uniswap_url: "http://127.0.0.1:9876".to_string(),
            liquidity_volume: 100.0,
            available_liquidity_seconds: 1000,
            unconditionally_valid_tokens: vec![addr("38A2fDc11f526Ddd5a607C1F251C065f40fBF2f7")],
            token_market_update_time: 1000,
            number_of_ticker_actors: 4,
            not_subsidized_tokens: vec![
                addr("2b591e99afe9f32eaa6214f7b7629768c40eeb39"),
                addr("34083bbd70d394110487feaa087da875a54624ec"),
            ],
        }
    }

    #[test]
    fn from_env() {
        let config = r#"
FEE_TICKER_TOKEN_PRICE_SOURCE="CoinGecko"
FEE_TICKER_COINMARKETCAP_BASE_URL="http://127.0.0.1:9876"
FEE_TICKER_COINGECKO_BASE_URL="http://127.0.0.1:9876"
FEE_TICKER_FAST_PROCESSING_COEFF="10"
FEE_TICKER_DISABLED_TOKENS="0x38A2fDc11f526Ddd5a607C1F251C065f40fBF2f7"
FEE_TICKER_NOT_SUBSIDIZED_TOKENS="0x2b591e99afe9f32eaa6214f7b7629768c40eeb39,0x34083bbd70d394110487feaa087da875a54624ec"
FEE_TICKER_UNISWAP_URL=http://127.0.0.1:9975/graphql
FEE_TICKER_TOKEN_AVAILABLE_LIQUIDITY_SECONDS=720
FEE_TICKER_TOKEN_TOKEN_MARKET_UPDATE_TIME_SECONDS=120
FEE_TICKER_UNCONDITIONALLY_VALID_TOKENS=0000000000000000000000000000000000000000
FEE_TICKER_LIQUIDITY_VOLUME=100
        "#;
        set_env(config);

        let actual = TickerConfig::from_env();
        assert_eq!(actual, expected_config());
    }

    /// Checks the correctness of the config helper methods.
    #[test]
    fn methods() {
        const COINGECKO_URL: &str = "http://coingecko";
        const COINMARKETCAP_URL: &str = "http://coinmarketcap";

        let mut config = expected_config();

        config.coingecko_base_url = COINGECKO_URL.into();
        config.coinmarketcap_base_url = COINMARKETCAP_URL.into();

        config.token_price_source = TokenPriceSource::CoinGecko;
        assert_eq!(
            config.price_source(),
            (TokenPriceSource::CoinGecko, COINGECKO_URL)
        );

        config.token_price_source = TokenPriceSource::CoinMarketCap;
        assert_eq!(
            config.price_source(),
            (TokenPriceSource::CoinMarketCap, COINMARKETCAP_URL)
        );
    }
}
