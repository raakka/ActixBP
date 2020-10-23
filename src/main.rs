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


