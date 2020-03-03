#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;


use actix_web::{get,post, web, App, HttpServer, Responder, middleware, Error, HttpResponse};
use actix_files as fs;

use std::sync::atomic::{AtomicU64, Ordering};

use lib_fbuffers;
use lib_comm;

//#[derive(Default)]
struct State{
    seq_h:AtomicU64,
    seq_l:AtomicU64,
}

impl Default for State{
    fn default() -> Self {
        State{seq_h:AtomicU64::new(0), seq_l:AtomicU64::new(0)}
    }
}



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


    let state = web::Data::new(State::default());

    HttpServer::new(move || 
        App::new()
        .wrap(
            middleware::Logger::default()
            .exclude("/health")
        )
        .data(client.clone())
        .app_data(state.clone())
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
async fn data(mut body: web::Payload, state: web::Data<State>, db: web::Data<lib_db::Pool>) -> Result<HttpResponse, Error>{
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let vbytes = bytes.to_vec();
    bytes.freeze();

    let envelop = lib_comm::get_root_as_message(&vbytes);

    {
        let seq = state.seq_h.swap(envelop.seq(), Ordering::Relaxed);
        if  seq >=  envelop.seq(){
            println!("Envelop: out of order cur:{} >= inc:{}, gap dn:{}", seq, envelop.seq(), seq - envelop.seq());
        }else if (seq+1) !=  envelop.seq() {
            println!("Envelop: out of order cur:{} >= inc:{}, gap up:{}", seq, envelop.seq(), envelop.seq() - seq);
        }
    }

    info!("sensor:{}, mseq:{}, uptime:{}, message_type:{}", 
        envelop.sensor_id().unwrap(),
        envelop.seq(),
        envelop.uptime(),
        envelop.ptype()
    );

    let payload = envelop.payload().unwrap();
    let vpayload:Vec<u8> = payload.into();

    //traceroute
    if envelop.ptype() == 1{
        //this will be offloaded to plugin
        let msg = lib_fbuffers::traceroute_generated::get_root_as_message(&vpayload);
        {
            let seq = state.seq_l.swap(msg.seq(), Ordering::Relaxed);
            if  seq >=  msg.seq(){
                println!("Message: out of order cur:{} >= inc:{}, gap dn:{}", seq, msg.seq(), seq - msg.seq());
            }else if (seq+1) !=  msg.seq() {
                println!("Message: out of order cur:{} >= inc:{}, gap up:{}", seq, msg.seq(), msg.seq() - seq);
            }
        }

        if let Some(routes) = msg.routes(){
            routes.iter().for_each(|r|{
                let route = lib_data::AppTraceRoute::new(
                    r.route_id(),
                    std::net::Ipv4Addr::from(r.src()),
                    std::net::Ipv4Addr::from(r.dst())
                );

                //println!("+++++++++++{}", route);
                lib_db::add_route_l(&route);
                // if let Ok(mut conn) = db.get_connection(){
                //     lib_db::add_route(&mut conn, &route);
                // }else{
                //     println!("Is database running? Error connecting to db...");
                // }
            })
        }else if let Some(hops) = msg.hops(){
            hops.iter().for_each(|r|{
                let hop = lib_data::AppHop::new(
                    r.route_id(),
                    std::net::Ipv4Addr::from(r.src()),
                    std::net::Ipv4Addr::from(r.this()),
                    r.ttl(),
                    r.rtt()
                );

                lib_db::add_hop_l(&hop);
                // //println!("==========\t{}", hop);
                // if let Ok(mut conn) = db.get_connection(){
                //     lib_db::add_hop(&mut conn, &hop);
                // }else{
                //     println!("Is database running? Error connecting to db...");
                // }
            })
        }

    }else if envelop.ptype() == 2{
        let msg = lib_fbuffers::allipv4_generated::get_root_as_message(&vpayload);
        if let Some(packets) = msg.packets(){
            packets.iter().for_each(|p|{
                println!("[{}] {} {}->{} [{}] {} {:?}", 
                    p.proto().unwrap(),
                    p.id(),
                    std::net::Ipv4Addr::from(p.src()),
                    std::net::Ipv4Addr::from(p.dst()),
                    p.len(),
                    bitflags_to_string(p.flags()),
                    p.opts().unwrap(),
            );
            
            })
        }

    }




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


use pnet::packet::tcp::TcpFlags;
bitflags! {
    struct Flags: u16 {
        const FIN = TcpFlags::FIN; //1
        const SYN = TcpFlags::SYN; //2
        const RST = TcpFlags::RST; //4
        const PSH = TcpFlags::PSH; //8
        const ACK = TcpFlags::ACK; //16
        const URG = TcpFlags::URG; //32

        const CWR = TcpFlags::CWR; //
        const ECE = TcpFlags::ECE; //
    }
}

fn bitflags_to_string(flags:u16) -> String{
    if let Some(s) = Flags::from_bits(flags) {
        return format!("{:?}", s);
    }
    String::new()
}


