const CONFIG_DEST: &str = "127.0.0.1:8089";
const CONFIG_QUERY_URL: &str = "/query";
const CONFIG_TRANSACTION_URL: &str = "/tx";
const CONFIG_SETUP_URL: &str = "/setup";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Config {
    pub dest: String,
    pub query_url: String,
    pub transaction_url: String,
    pub setup_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            dest: CONFIG_DEST.to_string(),
            query_url: CONFIG_QUERY_URL.to_string(),
            transaction_url: CONFIG_TRANSACTION_URL.to_string(),
            setup_url: CONFIG_SETUP_URL.to_string(),
        }
    }
}
