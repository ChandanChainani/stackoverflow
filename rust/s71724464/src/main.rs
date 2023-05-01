use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Serialize, Deserialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[derive(Serialize, Deserialize)]
pub struct Data {
   some_data: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<'a> {
   status: &'a str
}

#[post("/")]
async fn handler_post(
  request: HttpRequest,
  data: web::Json<Data>
) -> impl Responder {
  HttpResponse::Ok()
     .json(ApiResponse {
        status: "success"
     })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(handler_post)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
