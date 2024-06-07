use axum::{
    routing::post,
    Router,
    extract::Json,
};
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = SocketAddr::from(([0, 0, 0, 0], 3000)))]
    address: SocketAddr,

    #[arg(short, long, default_value_t = 10)]
    threads: usize
}


fn main() {
    let args = Args::parse();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on( async {
            let app = Router::new().route("/", post(my_handler));
            let listener = tokio::net::TcpListener::bind(args.address).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    
}

async fn my_handler(Json(payload): Json<serde_json::Value>) {

}