use dotenv::dotenv;
use lazy_static;
use std::env;
use std::ffi::OsString;

const CLI_PATH: &'static str = "CLI_PATH";
const BINDING: &'static str = "BINDING";
const CHANNEL: &'static str = "CHANNEL";
const CHAINCODE: &'static str = "CHAINCODE";

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        let cli_path = env::var_os(CLI_PATH).expect("Environment variable 'CLI_PATH' not set");
        let binding = env::var(BINDING).expect("Environment variable 'BINDING' not set");
        let channel = env::var_os(CHANNEL).expect("Environment variable 'CHANNEL' not set");
        let chaincode = env::var_os(CHAINCODE).expect("Environment variable 'CHAINCODE' not set");
        Config {
            cli_path: cli_path,
            binding_address: binding,
            channel: channel,
            chaincode: chaincode,
        }
    };
}

#[derive(Debug, Clone)]
pub struct Config {
    pub cli_path: OsString,
    pub binding_address: String,
    pub channel: OsString,
    pub chaincode: OsString,
}
