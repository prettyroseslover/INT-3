use pyo3::prelude::*;
use std::path::PathBuf;
use serde::{
    Serialize,
    Deserialize,
};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "command1", content = "params")]
pub enum Commands {
    CheckLocalFile(CheckLocalFileParams),
    QuarantineLocalFile(QuarantineLocalFileParams),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CheckLocalFileParams {
    pub path: PathBuf,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct QuarantineLocalFileParams {
    pub path: PathBuf
}

#[pyfunction]
fn serialize_check(path: &str, signature: Vec<u8>) -> PyResult<String> {
    let json_to_be: Commands = Commands::CheckLocalFile(
        CheckLocalFileParams {
        path: PathBuf::from(path),
        signature: signature
    });
    let result = serde_json::to_string(&json_to_be).unwrap();
    Ok(result)
}

#[pyfunction]
fn serialize_quarantine(path: &str) -> PyResult<String> {
    let json_to_be: Commands = Commands::QuarantineLocalFile(
        QuarantineLocalFileParams {
        path: PathBuf::from(path)
    });
    let result = serde_json::to_string(&json_to_be).unwrap();
    Ok(result)
}

#[pymodule]
#[pyo3(name="ptstart_int_3")]
fn python_export(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serialize_check, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_quarantine, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_check_local_file() {
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
    fn test_parse_quarantine_local_file() {
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