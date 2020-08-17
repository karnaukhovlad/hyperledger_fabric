mod actor;
mod config;
mod handlers;
mod source;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;
use crate::actor::CliActor;
use crate::config::CONFIG;
use crate::handlers::*;
use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpServer};
use env_logger;
use std::env;
use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let server = HttpServer::new(move || {
        let actor = CliActor::start(
            CONFIG.cli_path.clone(),
            CONFIG.channel.clone(),
            CONFIG.chaincode.clone(),
        );
        let tera_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*");
        dbg!(tera_path);
        let tera = Tera::new(tera_path).unwrap();
        App::new()
            .data(actor)
            .data(tera)
            .wrap(Cors::new().allowed_origin("*").send_wildcard().finish())
            .wrap(middleware::Logger::default())
            .service(web::resource("/{asset_id}").route(web::get().to(get_asset)))
            .service(web::resource("/").route(web::get().to(all_assets)))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(web::route().guard(guard::Not(guard::Get())).to(p404)),
            )
    })
    .bind(CONFIG.binding_address.clone())?
    .run();

    // run server
    server.await
}
