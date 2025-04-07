use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use linegen::LineGenerator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DEFAULT_PORT: u16 = 8080;
    let port = std::env::var("PORT")
        .map(|p| p.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);
    let listen_addr = format!("0.0.0.0:{}", port);

    println!("Listening on port :{}", listen_addr);
    HttpServer::new(move || App::new().service(gen_line))
        .bind(listen_addr.as_str())?
        .run()
        .await
}

#[get("/")]
async fn gen_line() -> impl Responder {
    let mut lg = LineGenerator::default();
    HttpResponse::Ok().body(lg.next().unwrap())
}
