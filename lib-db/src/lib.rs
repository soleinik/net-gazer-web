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


fn junk(){

    // let redis_url = opts.redis_url.clone().unwrap_or_else(||"redis://localhost/net-gazer".into());
    // info!("About to attempt to connect to '{}'...", redis_url);


    // let redis_url="redis://127.0.0.1";
    // let client = std::sync::Arc::new(redis::Client::open(redis_url).unwrap_or_else(|e|{
    //     error!("Redis connectivity failed! Error:{}",e);
    //     std::process::exit(-1);
    // }));

    // let data = key.clone();

    // task::spawn(async move {
    //     if let Err(e) = client.get_connection()
    //         .and_then(|mut conn| {

    //             redis::pipe()
    //                 .cmd("LPUSH")
    //                     .arg(key.clone())
    //                     .arg(data)
    //                     .ignore()
    //                 .cmd("EXPIRE")
    //                     .arg(key.clone())
    //                     .arg(1000 * 60)
    //                     .ignore()
    //             .query::<()>(&mut conn)

    //         }){
    //             error!("redis: unable to send! Error:{}", e);
    //         }
    
    // });

    // task::spawn(async move {

    //     if let Err(e) = client.get_connection()
    //         .and_then(|mut conn| {
    //             redis::cmd("LPUSH")
    //                 .arg(key.clone()).arg(data)
    //             .query::<()>(&mut conn)
    //         }){
    //             error!("redis: unable to send! Error:{}", e);
    //         }
    
    // });


}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
