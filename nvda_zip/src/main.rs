use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web};
use askama::Template;
use nvda_url::{NvdaUrl, VersionType, WIN7_URL, XP_URL};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

type SharedNvdaUrl = web::Data<Arc<Mutex<NvdaUrl>>>;

#[derive(Serialize)]
struct UrlResponse {
    url: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

async fn index(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Stable).await {
        Some(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        None => {
            HttpResponse::InternalServerError().body("Error fetching latest stable NVDA version")
        }
    }
}

async fn stable_json(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Stable).await {
        Some(url) => HttpResponse::Ok().json(UrlResponse { url }),
        None => HttpResponse::InternalServerError().json(UrlResponse { url: String::new() }),
    }
}

async fn redirect(url: &'static str) -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", url))
        .finish()
}

async fn json_response(url: &'static str) -> impl Responder {
    HttpResponse::Ok().json(UrlResponse { url: url.into() })
}

async fn alpha(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Alpha).await {
        Some(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        None => {
            HttpResponse::InternalServerError().body("Error fetching latest NVDA alpha version")
        }
    }
}

async fn alpha_json(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Alpha).await {
        Some(url) => HttpResponse::Ok().json(UrlResponse { url }),
        None => HttpResponse::InternalServerError().json(UrlResponse { url: String::new() }),
    }
}

async fn beta(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Beta).await {
        Some(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        None => HttpResponse::InternalServerError().body("Error fetching latest NVDA beta version"),
    }
}

async fn beta_json(nvda_url: SharedNvdaUrl) -> impl Responder {
    let nvda_url = nvda_url.lock().await;
    match nvda_url.get_url(VersionType::Beta).await {
        Some(url) => HttpResponse::Ok().json(UrlResponse { url }),
        None => HttpResponse::InternalServerError().json(UrlResponse { url: String::new() }),
    }
}

async fn not_found() -> impl Responder {
    match NotFoundTemplate.render() {
        Ok(body) => HttpResponse::NotFound()
            .content_type("text/html")
            .body(body),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering 404 page"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let nvda_url = Arc::new(Mutex::new(NvdaUrl::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(nvda_url.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/stable.json", web::get().to(stable_json))
            .route("/xp", web::get().to(|| redirect(XP_URL)))
            .route("/xp.json", web::get().to(|| json_response(XP_URL)))
            .route("/win7", web::get().to(|| redirect(WIN7_URL)))
            .route("/win7.json", web::get().to(|| json_response(WIN7_URL)))
            .route("/alpha", web::get().to(alpha))
            .route("/alpha.json", web::get().to(alpha_json))
            .route("/beta", web::get().to(beta))
            .route("/beta.json", web::get().to(beta_json))
            .default_service(web::to(not_found))
    })
    .bind_auto_h2c(("0.0.0.0", 5000))?
    .run()
    .await
}
