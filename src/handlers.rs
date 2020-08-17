use crate::actor::{AssetId, CliActor, GetAll};
use actix::Addr;
use actix_web::{error, Error, Result};
use actix_web::{web, HttpRequest, HttpResponse};

/// 404 handler
pub async fn p404(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let html = tmpl
        .render("404.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(html))
}

pub async fn get_asset(
    _: HttpRequest,
    cli: web::Data<Addr<CliActor>>,
    path: web::Path<(String,)>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let val = cli.send(AssetId(path.0.clone())).await.unwrap();
    let context = match val {
        Ok(asset) => {
            let mut ctx = tera::Context::new();
            ctx.insert("asset", &asset);
            ctx.insert("not_empty", &true);
            ctx
        }
        Err(_) => {
            let mut ctx = tera::Context::new();
            ctx.insert("not_empty", &false);
            ctx
        }
    };
    let html = tmpl
        .render("asset.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn all_assets(
    _: HttpRequest,
    cli: web::Data<Addr<CliActor>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let val = cli.send(GetAll()).await.unwrap();

    let context = match val {
        Ok(asset) => {
            let mut ctx = tera::Context::new();
            ctx.insert("table", &asset);
            ctx.insert("not_empty", &true);
            ctx
        }
        Err(_) => {
            let mut ctx = tera::Context::new();
            ctx.insert("not_empty", &false);
            ctx
        }
    };
    let html = tmpl
        .render("assets.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
