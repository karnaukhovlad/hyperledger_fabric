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
use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use std::env;

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
        App::new()
            .data(actor)
            .wrap(Cors::new().allowed_origin("*").send_wildcard().finish())
            .wrap(middleware::Logger::default())
            .service(web::resource("/{asset_id}").route(web::get().to(get_asset)))
            .service(web::resource("/").route(web::get().to(all_assets)))
        // .default_service(
        //     // 404 for GET request
        //     web::resource("")
        //         .route(web::get().to(p404))
        //         // all requests that are not `GET`
        //         .route(
        //             web::route()
        //                 .guard(guard::Not(guard::Get()))
        //                 .to(HttpResponse::MethodNotAllowed),
        //         ),
        // )
    })
    .bind(CONFIG.binding_address.clone())?
    .run();

    // run server
    server.await
}
