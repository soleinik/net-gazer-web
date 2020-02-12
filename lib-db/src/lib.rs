#[macro_use] extern crate log;


pub type Pool = std::sync::Arc<redis::Client>;

pub fn get_conn(opts:&lib_data::OptConf) -> Pool{
    let redis_url = opts.redis_url.clone().unwrap_or_else(||"redis://localhost".into());
    info!("About to attempt to connect to '{}'...", redis_url);

    std::sync::Arc::new(redis::Client::open(redis_url).unwrap_or_else(|e|{
        error!("Redis connectivity failed! Error:{}",e);
        std::process::exit(-1);
    }))
}

pub fn add_route(db:&Pool, pkt_id:u16, data:&[u8]){
    let res = db.get_connection()
        .and_then(|mut conn| {
            redis::cmd("LPUSH")
                    .arg(pkt_id)
                    .arg(data)
            .query::<()>(&mut conn)
        });

    if let Err(e) = res{
        error!("redis.routes: unable to send! Error:{}", e);
    }
}

pub fn add_hop(db:&Pool, pkt_id:u16, data:&[u8]){
    let res = db.get_connection()
        .and_then(|mut conn| {
            redis::cmd("LPUSH")
                    .arg(pkt_id)
                    .arg(data)
            .query::<()>(&mut conn)
        });

    if let Err(e) = res{
        error!("redis.hops: unable to send! Error:{}", e);
    }
}
