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

pub fn add_route(conn:& mut redis::Connection, route:&lib_data::AppTraceRoute){
    let create_0 = format!(
        "MERGE (h:hop {{ip:'{}'}}) \
            ON CREATE SET h.route_id={} 
        ",
        route.src, route.route_id
    );

    let create_1 = format!(
        "MERGE (h:hop {{ip:'{}'}}) \
            ON CREATE SET h.route_id={} 
        ",
        route.dst, route.route_id,
    );

    println!("{}", create_0);
    println!("{}", create_1);

    redis::pipe().
        cmd("GRAPH.QUERY")
            .arg("traceroute")
            .arg(create_0)
        .ignore()
        .cmd("GRAPH.QUERY")
            .arg("traceroute")
            .arg(create_1)
    .query::<()>(conn).unwrap();

}

pub fn add_hop(conn:& mut redis::Connection, hop:&lib_data::AppHop){
    let create_nodes = format!(
        "
        MERGE (h1:hop {{ip:'{}'}}) \
            ON CREATE SET h1.route_id={} \
        WITH h1 \
        MATCH (h2:hop{{ip:'{}'}}) \
        CREATE (h1)<-[r:next{{ ttl:{}, rtt:{} }}]-(h2)
        ",
        hop.this, 
        hop.route_id,
        hop.src,
        hop.ttl,
        hop.rtt,
    );
    println!("{}", create_nodes);

    redis::cmd("GRAPH.QUERY")
            .arg("traceroute")
            .arg(create_nodes)
        .query::<()>(conn).unwrap();
}
