use crate::actor::{AssetId, CliActor, GetAll};
use crate::source::cli::CliHandler;
use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse};

// /// 404 handler
// pub async fn p404() -> Result<fs::NamedFile> {
//     Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
// }

pub async fn get_asset(
    req: HttpRequest,
    cli: web::Data<Addr<CliActor>>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let val = cli.send(AssetId(path.0.clone())).await.unwrap();
    match val {
        Ok(asset) => HttpResponse::Ok().json(asset),
        Err(_) => HttpResponse::NotFound().body(format!("Asset with id {} not found.", path.0)),
    }
}

pub async fn all_assets(req: HttpRequest, cli: web::Data<Addr<CliActor>>) -> HttpResponse {
    let val = cli.send(GetAll()).await.unwrap();
    match val {
        Ok(asset) => HttpResponse::Ok().json(asset),
        Err(_) => HttpResponse::NotFound().json(format!("No assets.")),
    }
}
