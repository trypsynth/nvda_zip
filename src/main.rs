use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use regex::Regex;
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct UrlResponse {
    url: String,
}

async fn get_url(version_type: &str) -> Option<String> {
    let url = format!(
        "https://www.nvaccess.org/nvdaUpdateCheck?versionType={}",
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
                "https://www.nvaccess.org/download/nvda/releases/{}/nvda_{}.exe",
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
            "https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe",
        ))
        .finish()
}

async fn xp_json() -> impl Responder {
    HttpResponse::Ok().json(UrlResponse {
        url: "https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe".to_string(),
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
    let tera = Tera::new("templates/*").unwrap();
    let mut context = Context::new();
    context.insert("message", "The page you are looking for does not exist.");
    let rendered = tera.render("404.html", &context).unwrap();
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
