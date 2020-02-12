#[macro_use] extern crate log;


use actix_web::{get,post, web, App, HttpServer, Responder, middleware, Error, HttpResponse};
use actix_files as fs;

use lib_fbuffers::get_root_as_message;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_BACKTRACE", "1");

    //load from command line...
    let mut opt = lib_data::OptConf::default();

    //setup logger...
    match opt.verbosity{
        0 => std::env::set_var("RUST_LOG", "warn"),
        1 => std::env::set_var("RUST_LOG", "info"),
        2 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    }
    
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    //load from file...
    opt.load(env!("CARGO_PKG_NAME"));
    opt.validate().unwrap();

    let client = lib_db::get_conn(&opt);

    let url = format!("{}:{}", opt.http_ip.unwrap(), opt.http_port.unwrap());
    println!("starting server at {}...", url);

    HttpServer::new(move || 
        App::new()
        .wrap(
            middleware::Logger::default()
            .exclude("/health")
        )
        .data(client.clone())
        .service(chart)
        .service(data)
        .service(health)
        .service(fs::Files::new("chart", "./www/static"))
    )
    .bind(&url)?
    .run()
    .await
}


use futures::StreamExt;

#[post("/data")]
async fn data(mut body: web::Payload, db: web::Data<lib_db::Pool>) -> Result<HttpResponse, Error>{
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let vmsg = bytes.to_vec();
    let msg = get_root_as_message(&vmsg);

    if let Some(routes) = msg.routes(){
        routes.iter().for_each(|r|{
            let r = lib_data::AppTraceRoute::new(
                r.route_id(),
                std::net::Ipv4Addr::from(r.src()),
                std::net::Ipv4Addr::from(r.dst())
            );
            println!("{}", r);
        })
    }else if let Some(hops) = msg.hops(){
        hops.iter().for_each(|r|{
            let h = lib_data::AppHop::new(
                r.route_id(),
                std::net::Ipv4Addr::from(r.hop()),
                r.ttl(),
                r.rtt()
            );
            println!("\t{}", h);
        })
    }
    bytes.freeze();
    Ok(HttpResponse::Ok().finish())
}

#[get("/health")]
async fn health() ->  impl Responder {
    "UP"
    .with_header("Content-Type", "text/plain; charset=utf-8")
    .with_status(actix_web::http::StatusCode::OK)
}


#[get("/")]
async fn chart() -> Result<HttpResponse, Error> {
    // response
    Ok(HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../www/static/chart.html")))
}