mod models;
use actix_web::{ web, App, HttpServer, Responder, HttpResponse};
use models::Stutus;
use std::io;

async fn status() -> impl Responder {
      HttpResponse::Ok()
      .json(Stutus{status:"UP".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()>{

   println!("server run ..");
   HttpServer::new(||{
      App::new()
            .route("/", web::get().to(status))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
