use actix_web::{
    	error,
	middleware,
	web,
	App,
	Error,
	HttpRequest,
	HttpResponse,
	HttpServer
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::io;

// json obj we expect the client to post
#[derive(Debug, Serialize, Deserialize)]
struct PostJsonObj {
	site: String
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Handlers

async fn json_handler(
	req_obj: web::Json<PostJsonObj>,
	_req: HttpRequest,
) -> HttpResponse {
	HttpResponse::Ok().json(req_obj.0)
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// Main

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.service(web::resource("/ticket")
				.data(web::JsonConfig::default().limit(512))
				.route(web::post().to(json_handler))
			)
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}
