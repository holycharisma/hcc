use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SessionUser {
    pub email: String
}