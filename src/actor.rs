use std::collections::BTreeMap;

use actix::prelude::*;
// use failure::Error;
use futures::{FutureExt, TryFutureExt};

use crate::source::cli::Command;
use crate::source::{cli::CliHandler, Asset, AssetsList};
use std::ffi::{OsStr, OsString};
use std::ops::Deref;
use std::path::Path;

pub struct CliActor {
    cache: BTreeMap<String, Asset>,
    path: OsString,
    channel: OsString,
    chaincode: OsString,
}

pub struct GetAll();

impl Message for GetAll {
    type Result = Result<Vec<AssetsList>, Error>;
}

pub struct AssetId(pub String);

impl Message for AssetId {
    type Result = Result<Asset, Error>;
}

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Value not found")]
    NotFound,
}

impl actix_web::ResponseError for Error {}

impl CliActor {
    pub fn start(path: OsString, channel: OsString, chaincode: OsString) -> Addr<CliActor> {
        Supervisor::start(|_| CliActor {
            cache: Default::default(),
            path,
            channel,
            chaincode,
        })
    }
}

impl Actor for CliActor {
    type Context = Context<Self>;
}

impl Supervised for CliActor {
    fn restarting(&mut self, _: &mut Self::Context) {
        info!("restarting");
    }
}

impl Handler<GetAll> for CliActor {
    type Result = ResponseFuture<Result<Vec<AssetsList>, Error>>;

    fn handle(&mut self, msg: GetAll, _: &mut Self::Context) -> Self::Result {
        let mut cli_handler = CliHandler::init(
            self.path.clone(),
            self.channel.clone(),
            self.chaincode.clone(),
        );
        let future = async move { cli_handler.get_all().await };

        Box::pin(future.map(|val| val.ok_or(Error::NotFound)))
    }
}

impl Handler<AssetId> for CliActor {
    type Result = ResponseFuture<Result<Asset, Error>>;

    fn handle(&mut self, msg: AssetId, _: &mut Self::Context) -> Self::Result {
        let mut cli_handler = CliHandler::init(
            self.path.clone(),
            self.channel.clone(),
            self.chaincode.clone(),
        );
        let future = async move { cli_handler.get_asset(msg.0).await };

        Box::pin(future.map(|val| val.ok_or(Error::NotFound)))
    }
}
