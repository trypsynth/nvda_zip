use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web};
use askama::Template;
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct UrlResponse {
    url: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    message: String,
}

async fn get_url(version_type: &str) -> Option<String> {
    let url = format!(
        "https://download.nvaccess.org/nvdaUpdateCheck?versionType={}",
        version_type
    );
    let response = reqwest::get(&url).await.ok()?;
    let body = response.text().await.ok()?;
    let regex = match version_type {
        "snapshot:alpha" => Regex::new(r"launcherUrl:\s*(.*)").ok()?,
        "beta" | "stable" => Regex::new(r"version:\s*(.*)").ok()?,
        _ => return None,
    };
    let captured = regex.captures(&body)?;
    match version_type {
        "snapshot:alpha" => captured.get(1).map(|m| m.as_str().to_string()),
        "beta" | "stable" => {
            let version = captured.get(1)?.as_str().trim();
            Some(format!(
                "https://download.nvaccess.org/download/releases/{}/nvda_{}.exe",
                version, version
            ))
        }
        _ => None,
    }
}

async fn index() -> impl Responder {
    if let Some(url) = get_url("stable").await {
        HttpResponse::Found()
            .append_header(("Location", url))
            .finish()
    } else {
        HttpResponse::InternalServerError()
            .body("There was an error getting the latest stable NVDA version")
    }
}

async fn stable_json() -> impl Responder {
    if let Some(url) = get_url("stable").await {
        HttpResponse::Ok().json(UrlResponse { url })
    } else {
        HttpResponse::InternalServerError().body("{}")
    }
}

async fn xp() -> impl Responder {
    HttpResponse::Found()
        .append_header((
            "Location",
            "https://download.nvaccess.org/download/releases/2017.3/nvda_2017.3.exe",
        ))
        .finish()
}

async fn xp_json() -> impl Responder {
    HttpResponse::Ok().json(UrlResponse {
        url: "https://download.nvaccess.org/download/releases/2017.3/nvda_2017.3.exe".to_string(),
    })
}

async fn alpha() -> impl Responder {
    if let Some(url) = get_url("snapshot:alpha").await {
        HttpResponse::Found()
            .append_header(("Location", url))
            .finish()
    } else {
        HttpResponse::InternalServerError()
            .body("There was an error getting the latest NVDA alpha version")
    }
}

async fn alpha_json() -> impl Responder {
    if let Some(url) = get_url("snapshot:alpha").await {
        HttpResponse::Ok().json(UrlResponse { url })
    } else {
        HttpResponse::InternalServerError().body("{}")
    }
}

async fn beta() -> impl Responder {
    if let Some(url) = get_url("beta").await {
        HttpResponse::Found()
            .append_header(("Location", url))
            .finish()
    } else {
        HttpResponse::InternalServerError().body("There was an error getting the latest NVDA beta")
    }
}

async fn beta_json() -> impl Responder {
    if let Some(url) = get_url("beta").await {
        HttpResponse::Ok().json(UrlResponse { url })
    } else {
        HttpResponse::InternalServerError().body("{}")
    }
}

async fn not_found() -> impl Responder {
    let not_found_template = NotFoundTemplate {
        message: String::from("The page you're looking for does not exist."),
    };
    let rendered = not_found_template.render().unwrap();
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/stable.json", web::get().to(stable_json))
            .route("/xp", web::get().to(xp))
            .route("/xp.json", web::get().to(xp_json))
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
