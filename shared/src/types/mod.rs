use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ZkCommit {
    pub has_error: bool,
    pub err_msg: String,
    pub cred_hashes: Vec<String>,
    pub script: String,
    pub result: bool,
}

#[derive(Serialize, Deserialize)]
pub enum ScriptLang {
    Rhai,
    JavaScript,
}
