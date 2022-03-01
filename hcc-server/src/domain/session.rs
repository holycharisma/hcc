use tide::prelude::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SessionUser {
    pub email: String
}