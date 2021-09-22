#[derive(Debug, Clone)]
pub enum Endpoint {
    Com,
    Us,
}

impl Endpoint {
    pub fn ws(&self) -> &'static str {
        match self {
            Endpoint::Com => "wss://ftx.com/ws",
            Endpoint::Us => "wss://ftx.us/ws",
        }
    }

    pub fn rest(&self) -> &'static str {
        match self {
            Endpoint::Com => "https://ftx.com/api",
            &Endpoint::Us => "https://ftx.us/api",
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

    pub fn authenticate(mut self, key: String, secret: String) -> Self {
        self.key = Some(key);
        self.secret = Some(secret);
        self
    }

    pub fn subaccount(mut self, subaccount: String) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn subaccount_optional(mut self, subaccount: Option<String>) -> Self {
        self.subaccount = subaccount;
        self
    }
}
