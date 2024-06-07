use std::path::PathBuf;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "command1", content = "params")]
enum Commands {
    CheckLocalFile(CheckLocalFileParams),
    QuarantineLocalFile(QuarantineLocalFileParams),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CheckLocalFileParams {
    path: PathBuf,
    signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct QuarantineLocalFileParams {
    path: PathBuf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_local_file() {
        let path = "/home/utente";
        let signature = Vec::from([5, 8, 0, 0, 8]);
        let json = format!(r#"
        {{
            "command1": "CheckLocalFile",
            "params": {{
                "path": "{path}", 
                "signature": [5, 8, 0, 0, 8]
            }}  
        }}"#);

        let got: Commands = serde_json::from_str(&json).unwrap();
        let expected: Commands = Commands::CheckLocalFile(
            CheckLocalFileParams {
            path: PathBuf::from(path),
            signature: signature
        });

        assert_eq!(got, expected);
    }

    #[test]
    fn test_quarantine_local_file() {
        let path = "/home/utente";
        let json = format!(r#"
        {{
            "command1": "QuarantineLocalFile",
            "params": {{
                "path": "{path}"
            }}
        }}"#);

        let got: Commands = serde_json::from_str(&json).unwrap();
        let expected: Commands = Commands::QuarantineLocalFile(
            QuarantineLocalFileParams {
            path: PathBuf::from(path)
        });

        assert_eq!(got, expected);
    }
}