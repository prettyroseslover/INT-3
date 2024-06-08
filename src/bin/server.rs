use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use clap::Parser;
use eyre::{eyre, Result};
use ptstart_int_3::{CheckLocalFileParams, Commands, QuarantineLocalFileParams};
use serde_json::{json, Value};
use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = SocketAddr::from(([0, 0, 0, 0], 3000)))]
    address: SocketAddr,

    #[arg(short, long, default_value_t = 10)]
    threads: usize,

    #[arg(short, long)]
    quarantine: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if !(args.quarantine.exists() && args.quarantine.is_dir()) {
        return Err(eyre!(
            "{:?} either doesn't exist or is not a directory",
            args.quarantine
        ));
    }

    let shared_state = Arc::new(args.quarantine);

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let app = Router::new().route("/", post(my_handler).with_state(shared_state));
            let listener = tokio::net::TcpListener::bind(args.address).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });

    Ok(())
}

async fn my_handler(
    State(quarantine): State<Arc<PathBuf>>,
    Json(payload): Json<Commands>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match payload {
        Commands::CheckLocalFile(params) => check_local_file(params),
        Commands::QuarantineLocalFile(params) => quarantine_local_file(params, quarantine),
    }
}

fn check_local_file(params: CheckLocalFileParams) -> Result<Json<Value>, (StatusCode, String)> {
    if !(params.path.exists() && params.path.is_file()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("{:?} either doesn't exist or is not a file", params.path).to_string(),
        ));
    };
    Ok(Json(json!({ "payload": "result" }))) // change
}

fn quarantine_local_file(
    params: QuarantineLocalFileParams,
    quarantine: Arc<PathBuf>,
) -> Result<Json<Value>, (StatusCode, String)> {
    if !(params.path.exists() && params.path.is_file()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("{:?} either doesn't exist or is not a file", params.path).to_string(),
        ));
    };

    let mut quarantine = quarantine.as_ref().clone();
    quarantine.set_file_name(params.path.file_name().unwrap());

    match fs::rename(&params.path, quarantine) {
        Ok(_) => Ok(Json(
            json!({ "message": format!("{:?} was quarantined", params.path) }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong while moving {:?}: {}", params.path, e).to_string(),
        )),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_quarantine_local_file() {
        let temp_quarantine = tempfile::tempdir().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let temp_path = dir.path().join("to_be_checked.txt");
        fs::File::create(&temp_path).unwrap();
        let quarantine = Arc::new(PathBuf::from(temp_quarantine.path()));
        let params = QuarantineLocalFileParams {
            path: PathBuf::from(&temp_path)
        };

        let got = quarantine_local_file(params, quarantine).unwrap().0;
        let expected = json!({ "message": format!("{:?} was quarantined", temp_path)});

        assert_eq!(got, expected);
    }

}