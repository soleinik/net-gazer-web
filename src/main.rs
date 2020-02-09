use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_BACKTRACE", "1");

    //read command line...
    let mut opt = lib_data::OptConf::default();

    //setup logger...
    match opt.verbosity{
        0 => std::env::set_var("RUST_LOG", "warn"),
        1 => std::env::set_var("RUST_LOG", "info"),
        2 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),

    }
    env_logger::init();

    //load from file...
    opt.load(env!("CARGO_PKG_NAME"));
    opt.validate().unwrap();






    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
