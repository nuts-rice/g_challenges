use std::fmt;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrabbooruError {
    pub message: String,
}

impl fmt::Display for CrabbooruError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.message)
    }
}

impl From<&str> for CrabbooruError {
    fn from(s: &str) -> Self {
        CrabbooruError {
            message: s.to_string(),
        }
    }
}
