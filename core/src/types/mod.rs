use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ZkCommit {
    pub has_error: bool,
    pub err_msg: String,
    pub script: String,
    pub result: bool,
}
