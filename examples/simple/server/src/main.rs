// use actix_web::{get, web, App, HttpServer, Responder};
// use serde_json::json;

// #[get("/api/test")]
// async fn index() -> impl Responder {
// 	web::Json(json!({
// 		"ok": true
// 	}))
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
// 	HttpServer::new(|| App::new().service(index))
// 		.bind("127.0.0.1:3000")?
// 		.run()
// 		.await
// }

fn main() {}
