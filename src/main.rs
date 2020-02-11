#[macro_use] extern crate log;


use actix_web::{get,post, web, App, HttpServer, Responder, middleware, Error, HttpResponse};

use lib_fbuffers::get_root_as_message;


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
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    //load from file...
    opt.load(env!("CARGO_PKG_NAME"));
    opt.validate().unwrap();


    let redis_url = opt.redis_url.clone().unwrap_or_else(||"redis://127.0.0.1".into());
    info!("About to attempt to connect to '{}'...", redis_url);

    let client = std::sync::Arc::new(redis::Client::open(redis_url).unwrap_or_else(|e|{
        error!("Redis connectivity failed! Error:{}",e);
        std::process::exit(-1);
    }));

    HttpServer::new(move || 
        App::new()
        .wrap(middleware::Logger::default())
        .data(client.clone())
        .service(index)
        .service(data)
    )
    .bind(format!("{}:{}", opt.http_ip.unwrap(), opt.http_port.unwrap()))?
    .run()
    .await
}

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

use futures::StreamExt;

#[post("/data")]
async fn data(mut body: web::Payload) -> Result<HttpResponse, Error>{
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let vmsg = bytes.to_vec();
    let msg = get_root_as_message(&vmsg);

    if let Some(routes) = msg.routes(){
        routes.iter().for_each(|r|println!(
            "id:{} src:{}->dst:{}",
            r.route_id(),
            std::net::Ipv4Addr::from(r.src()),
            std::net::Ipv4Addr::from(r.dst())
        ));
    }else if let Some(hops) = msg.hops(){
        hops.iter().for_each(|r|println!(
            "   id:{} hop:{} ttl:{} rtt:{} msec",
            r.route_id(),
            std::net::Ipv4Addr::from(r.hop()),
            r.ttl(),
            r.rtt()
        ));
    }
    Ok(HttpResponse::Ok().finish())
}
