use std::env::var;

use const_format::concatcp;

#[derive(Debug, Clone)]
pub enum Endpoint {
    Com,
    Us,
}

const ENDPOINT_HDR_PREFIX_COM: &str = "FTX-";
const ENDPOINT_HDR_PREFIX_US: &str = "FTXUS-";

impl Endpoint {
    pub const fn ws(&self) -> &'static str {
        match self {
            Endpoint::Com => "wss://ftx.com/ws",
            Endpoint::Us => "wss://ftx.us/ws",
        }
    }

    pub const fn rest(&self) -> &'static str {
        match self {
            Endpoint::Com => "https://ftx.com/api",
            Endpoint::Us => "https://ftx.us/api",
        }
    }
    #[cfg(feature = "optimized-access")]
    pub const fn optimized_access_rest(&self) -> &'static str {
        match self {
            Endpoint::Com => "https://api.ftx.com/api",
            Endpoint::Us => "https://ftx.us/api",
        }
    }
    pub const fn header_prefix(&self) -> &'static str {
        match self {
            Endpoint::Com => "FTX",
            Endpoint::Us => "FTXUS",
        }
    }

    pub const fn timestamp_header(&self) -> &'static str {
        match self {
            Endpoint::Com => concatcp!(ENDPOINT_HDR_PREFIX_COM, "TS"),
            Endpoint::Us => concatcp!(ENDPOINT_HDR_PREFIX_US, "TS"),
        }
    }

    pub const fn sign_header(&self) -> &'static str {
        match self {
            Endpoint::Com => concatcp!(ENDPOINT_HDR_PREFIX_COM, "SIGN"),
            Endpoint::Us => concatcp!(ENDPOINT_HDR_PREFIX_US, "SIGN"),
        }
    }

    pub const fn subaccount_header(&self) -> &'static str {
        match self {
            Endpoint::Com => concatcp!(ENDPOINT_HDR_PREFIX_COM, "SUBACCOUNT"),
            Endpoint::Us => concatcp!(ENDPOINT_HDR_PREFIX_US, "SUBACCOUNT"),
        }
    }

    pub const fn key_header(&self) -> &'static str {
        match self {
            Endpoint::Com => concatcp!(ENDPOINT_HDR_PREFIX_COM, "KEY"),
            Endpoint::Us => concatcp!(ENDPOINT_HDR_PREFIX_US, "KEY"),
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint::Com
    }
}

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub endpoint: Endpoint,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub subaccount: Option<String>,
}

impl Options {
    pub fn us() -> Self {
        Options {
            endpoint: Endpoint::Us,
            ..Default::default()
        }
    }

    pub fn from_env() -> Self {
        Options::default()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok())
    }

    pub fn from_env_us() -> Self {
        Options::us()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok())
    }

    #[must_use]
    pub fn authenticate(mut self, key: String, secret: String) -> Self {
        self.key = Some(key);
        self.secret = Some(secret);
        self
    }

    #[must_use]
    pub fn subaccount(mut self, subaccount: String) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    #[must_use]
    pub fn subaccount_optional(mut self, subaccount: Option<String>) -> Self {
        self.subaccount = subaccount;
        self
    }
}
