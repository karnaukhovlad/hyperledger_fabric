use crate::source::{Asset, AssetsList};
use serde::{Deserialize, Serialize};
use serde_json;
use std::ffi::{OsStr, OsString};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::{env, io};
use tokio::process::Command as TokioCommand;

#[derive(Debug)]
pub struct CliHandler {
    command: TokioCommand,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde(rename = "Args")]
    pub args: Vec<String>,
}

impl CliHandler {
    pub fn init(path: OsString, channel: OsString, chaincode: OsString) -> Self {
        let env_required: Vec<&str> = vec![
            "FABRIC_CFG_PATH",
            "CORE_PEER_ADDRESS",
            "CORE_PEER_MSPCONFIGPATH",
            "CORE_PEER_TLS_ROOTCERT_FILE",
            "CORE_PEER_LOCALMSPID",
            "CORE_PEER_TLS_ENABLED",
        ];
        let env_collection = env_required.into_iter().map(|env_key| {
            let env_val =
                env::var_os(env_key).expect(&format!("Environment variable '{}' not set", env_key));
            (env_key, OsString::from(env_val))
        });
        let mut full_path = PathBuf::new();
        full_path.push(path);
        full_path.push("peer");
        let mut exec = TokioCommand::new(&full_path);
        exec.envs(env_collection)
            .arg("chaincode")
            .arg("query")
            .arg("-C")
            .arg(channel)
            .arg("-n")
            .arg(chaincode)
            .arg("-c");

        Self { command: exec }
    }

    pub fn execute(&mut self, command: Command) -> impl Future<Output = io::Result<Output>> {
        let full_comm = self.command.arg(serde_json::to_string(&command).unwrap());
        full_comm.output()
    }

    pub async fn get_asset(&mut self, asset_id: String) -> Option<Asset> {
        let command = Command {
            args: vec!["ReadAsset".to_string(), asset_id],
        };
        let output = self.execute(command).await.unwrap();
        let stdout = output.stdout;
        if stdout.is_empty() {
            return None;
        };
        Some(serde_json::from_slice(&stdout).unwrap())
    }

    pub async fn get_all(&mut self) -> Option<Vec<AssetsList>> {
        let command = Command {
            args: vec!["GetAllAssets".to_string()],
        };
        let output = self.execute(command).await.unwrap();
        let stdout = output.stdout;
        if stdout.is_empty() {
            return None;
        };
        Some(serde_json::from_slice(&stdout).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::source::cli::{CliHandler, Command};
    use crate::source::Asset;
    use dotenv::dotenv;
    use std::str;

    #[tokio::test]
    async fn check_command_formatter() {
        let command = Command {
            args: vec!["ReadAsset".to_string(), "asset1".to_string()],
        };
        let json_view = r#"{"Args":["ReadAsset","asset1"]}"#;
        let formatted_command = serde_json::to_string(&command).unwrap();
        assert_eq!(json_view, formatted_command);
    }

    #[tokio::test]
    async fn check_command_execution() {
        dotenv().ok();
        let mut exec = CliHandler::init(
            &"/home/vlad/test_fabric/fabric-samples/bin".to_string(),
            &"mychannel".to_string(),
            &"basic".to_string(),
        );
        let command = Command {
            args: vec!["ReadAsset".to_string(), "asset1".to_string()],
        };
        let output = exec.execute(&command).await.unwrap();
        assert!(output.status.success());
        let stdout = output.stdout;
        let val = str::from_utf8(&stdout).unwrap();
        let right_val = r#"{"ID":"asset1","Color":"blue","Size":5,"Owner":"Tomoko","AppraisedValue":300,"docType":"asset"}
"#;
        assert_eq!(right_val, val);
    }

    #[tokio::test]
    async fn check_command_execution_on_struct() {
        dotenv().ok();
        let mut exec = CliHandler::init(
            &"/home/vlad/test_fabric/fabric-samples/bin".to_string(),
            &"mychannel".to_string(),
            &"basic".to_string(),
        );
        let command = Command {
            args: vec!["ReadAsset".to_string(), "asset1".to_string()],
        };
        let output = exec.execute(&command).await.unwrap();
        assert!(output.status.success());
        let stdout = output.stdout;
        let asset: Asset = serde_json::from_slice(&stdout).unwrap();
    }

    #[tokio::test]
    async fn check_value_not_find() {
        dotenv().ok();
        let mut exec = CliHandler::init(
            &"/home/vlad/test_fabric/fabric-samples/bin".to_string(),
            &"mychannel".to_string(),
            &"basic".to_string(),
        );
        let command = Command {
            args: vec!["ReadAsset".to_string(), "asset15".to_string()],
        };
        let output = exec.execute(&command).await.unwrap();
        let stdout = output.stdout;
        let val = str::from_utf8(&stdout).unwrap();
        let right_val = r#"{"ID":"asset1","Color":"blue","Size":5,"Owner":"Tomoko","AppraisedValue":300,"docType":"asset"}
"#;
        assert_ne!(right_val, val);
        assert_eq!(val, "");
    }

    #[tokio::test]
    async fn check_get_all() {
        dotenv().ok();
        let mut exec = CliHandler::init(
            &"/home/vlad/test_fabric/fabric-samples/bin".to_string(),
            &"mychannel".to_string(),
            &"basic".to_string(),
        );
        let assets = exec.get_all().await.unwrap();
        if assets.is_empty() {
            panic!("assets is empty")
        };
    }
}
