#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer, Result as ActixResult, middleware, web};
use askama::Template;
use nvda_url::{NvdaUrl, VersionType, WIN7_HASH, WIN7_URL, XP_HASH, XP_URL};
use serde::Serialize;
use tokio::sync::Mutex;

type SharedNvdaUrl = web::Data<Arc<Mutex<NvdaUrl>>>;

#[derive(Serialize)]
struct UrlResponse {
	url: String,
	hash: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

fn redirect_to(url: &str) -> HttpResponse {
	HttpResponse::Found().append_header(("Location", url)).finish()
}

fn json_url_response(url: String, hash: String) -> HttpResponse {
	HttpResponse::Ok().json(UrlResponse { url, hash })
}

async fn version_handler(
	nvda_url: SharedNvdaUrl,
	version_type: VersionType,
	as_json: bool,
) -> ActixResult<HttpResponse> {
	let nvda_url = nvda_url.lock().await;
	(nvda_url.get_details(version_type).await).map_or_else(
		|| {
			if as_json {
				Ok(HttpResponse::InternalServerError().json(UrlResponse { url: String::new(), hash: String::new() }))
			} else {
				Ok(HttpResponse::InternalServerError().body("Error fetching latest NVDA version"))
			}
		},
		|(url, hash)| {
			if as_json { Ok(json_url_response(url, hash)) } else { Ok(redirect_to(&url)) }
		},
	)
}

async fn index(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Stable, false).await
}

async fn stable_json(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Stable, true).await
}

async fn alpha(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Alpha, false).await
}

async fn alpha_json(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Alpha, true).await
}

async fn beta(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Beta, false).await
}

async fn beta_json(nvda_url: SharedNvdaUrl) -> ActixResult<HttpResponse> {
	version_handler(nvda_url, VersionType::Beta, true).await
}

async fn xp_redirect() -> ActixResult<HttpResponse> {
	Ok(redirect_to(XP_URL))
}

async fn xp_json() -> ActixResult<HttpResponse> {
	Ok(json_url_response(XP_URL.to_string(), XP_HASH.to_string()))
}

async fn win7_redirect() -> ActixResult<HttpResponse> {
	Ok(redirect_to(WIN7_URL))
}

async fn win7_json() -> ActixResult<HttpResponse> {
	Ok(json_url_response(WIN7_URL.to_string(), WIN7_HASH.to_string()))
}

async fn not_found() -> ActixResult<HttpResponse> {
	NotFoundTemplate.render().map_or_else(
		|_| Ok(HttpResponse::InternalServerError().body("Error rendering 404 page")),
		|body| Ok(HttpResponse::NotFound().content_type("text/html").body(body)),
	)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	let nvda_url = web::Data::new(Arc::new(Mutex::new(NvdaUrl::default())));
	HttpServer::new(move || {
		App::new()
			.app_data(nvda_url.clone())
			.wrap(middleware::Logger::default())
			.service(
				web::scope("")
					.route("/", web::get().to(index))
					.route("/stable.json", web::get().to(stable_json))
					.route("/alpha", web::get().to(alpha))
					.route("/alpha.json", web::get().to(alpha_json))
					.route("/beta", web::get().to(beta))
					.route("/beta.json", web::get().to(beta_json))
					.route("/xp", web::get().to(xp_redirect))
					.route("/xp.json", web::get().to(xp_json))
					.route("/win7", web::get().to(win7_redirect))
					.route("/win7.json", web::get().to(win7_json)),
			)
			.default_service(web::to(not_found))
	})
	.bind_auto_h2c(("0.0.0.0", 5000))?
	.run()
	.await
}
