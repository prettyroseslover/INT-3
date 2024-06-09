use clap::{Parser, Subcommand};
use reqwest::blocking as rq;
use serde_json::{to_value, Value};
use std::{io, path::PathBuf};
use std::io::prelude::*;
use ptstart_int_3::{CheckLocalFileParams, Commands, QuarantineLocalFileParams};
use eyre::{eyre, Result};
use http::StatusCode;
use color_print::cprintln;

#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// server address
    #[arg(short, long, default_value_t = String::from("http://127.0.0.1:3000/"))]
    url: String,
}

#[derive(Subcommand, Debug)]
#[command(arg_required_else_help(true))]
enum Command {
    #[command(arg_required_else_help(true))]
    CheckLocalFile {
        #[arg(short, long)]
        remote_path: PathBuf
    },

    #[command(arg_required_else_help(true))]
    QuarantineLocalFile {
        #[arg(short, long)]
        remote_path: PathBuf,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Command::CheckLocalFile { remote_path } => {
            let mut stdin = io::stdin();
            let mut signature = Vec::new();
            stdin.read_to_end(&mut signature)
                .map_err(|e| eyre!("Unable to read signature: {}", e))?;
            if signature.len() > 1024 {
                return Err(eyre!("Signature must not exceed 1 KiB"));
            }
            check_local_file(cli.url, remote_path, signature)?;
        },
        Command::QuarantineLocalFile { remote_path } => quarantine_local_file(cli.url, remote_path)?,
    }

    Ok(())
}

fn check_local_file(url: String, remote_path: PathBuf, signature: Vec<u8>) -> Result<()> {
    let json_to_be: Commands = Commands::CheckLocalFile(
        CheckLocalFileParams {
        path: remote_path,
        signature: signature
    });

    match send_reqwest(to_value(json_to_be)?, url) {
        Ok(value) => {
            let message = value["offset"].as_array().ok_or(eyre!("Not a string!"))?.to_owned()
                .into_iter().map(|num| num.as_number().and_then(|n| n.as_u64()))
                .collect::<Option<Vec<_>>>().ok_or(eyre!("Not a number!"))?;
            cprintln!("<green>{:?}</green>", message);
        },
        Err(e) => cprintln!("<red>{}</red>", e),
    }
    
    Ok(())
}

fn quarantine_local_file(url: String, remote_path: PathBuf) -> Result<()> {
    let json_to_be: Commands = Commands::QuarantineLocalFile(
        QuarantineLocalFileParams {
        path: remote_path
    });

    match send_reqwest(to_value(json_to_be)?, url) {
        Ok(value) => {
            let message = value["message"].as_str().ok_or(eyre!("Not a string!"))?.to_owned();
            cprintln!("<green>{}</green>", message);
        },
        Err(e) => cprintln!("<red>{}</red>", e),
    }

    Ok(())
}

fn send_reqwest(json_to_be: Value, url: String) -> Result<Value>{
    let client = rq::Client::new();
    let response = client.post(url)
        .json(&json_to_be)
        .send()
        .map_err(|e| eyre!("Error making reqwest: {}", e))?;

    match response.status() {
        StatusCode::OK => {
            return response.json::<Value>().map_err(|e| eyre!("Error while decoding json: {}", e))
        },
        _ => return Err(eyre!("Response with status {}: {:?}", response.status(), io::read_to_string(response))),
    }
}