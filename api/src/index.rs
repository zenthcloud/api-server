// Cargo.toml dépendances importantes à ajouter :
// actix-web = "4"
// actix-web-actors = "4"
// env_logger = "0.9"
// log = "0.4"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware};
use actix_web::dev::{ServiceRequest, ServiceResponse, forward_ready};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::Error;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::collections::HashSet;
use std::sync::Arc;
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use chrono::Utc;
use serde::Serialize;

struct ApiKeyMiddleware {
    valid_api_keys: Arc<HashSet<String>>,
}

impl<S, B> middleware::Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddlewareMiddleware {
            service,
            valid_api_keys: self.valid_api_keys.clone(),
        }))
    }
}

struct ApiKeyMiddlewareMiddleware<S> {
    service: S,
    valid_api_keys: Arc<HashSet<String>>,
}

impl<S, B> actix_service::Service<ServiceRequest> for ApiKeyMiddlewareMiddleware<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let valid_api_keys = self.valid_api_keys.clone();

        // Vérification de la clé API dans le header "x-api-key"
        if let Some(api_key) = req.headers().get("x-api-key").and_then(|h| h.to_str().ok()) {
            if valid_api_keys.contains(api_key) {
                // Clé valide, on continue
                let fut = self.service.call(req);
                return Box::pin(async move { fut.await });
            }
        }

        // Sinon, on renvoie Unauthorized
        let (req, _pl) = req.into_parts();
        let response = HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized: Invalid API Key"
        }));

        Box::pin(async move { Ok(ServiceResponse::new(req, response)) })
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    time: String,
}

async fn health() -> impl Responder {
    let response = HealthResponse {
        status: "ok",
        time: Utc::now().to_rfc3339(),
    };
    HttpResponse::Ok().json(response)
}

#[derive(Serialize)]
struct UserInfoResponse {
    user: &'static str,
    permissions: Vec<&'static str>,
    roles: Vec<&'static str>,
}

async fn user_info() -> impl Responder {
    let response = UserInfoResponse {
        user: "client123",
        permissions: vec!["read", "write"],
        roles: vec!["user"],
    };
    HttpResponse::Ok().json(response)
}

// Gestion WebSocket basique
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if text == "ping" {
                    ctx.text("pong");
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("Nouvelle connexion WebSocket");
    ws::start(MyWs {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger
    env_logger::init();

    let valid_api_keys: HashSet<String> = ["clé-api-1", "clé-api-2"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let api_key_middleware = ApiKeyMiddleware {
        valid_api_keys: Arc::new(valid_api_keys),
    };

    let port = std::env::var("PORT").unwrap_or_else(|_| "4000".to_string());

    println!("API Zenth Cloud démarrée sur le port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .service(web::resource("/health").route(web::get().to(health)))
            .service(
                web::scope("/api")
                    .wrap(api_key_middleware.clone())
                    .route("/user-info", web::get().to(user_info)),
            )
            .route("/ws/", web::get().to(websocket_handler))
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
