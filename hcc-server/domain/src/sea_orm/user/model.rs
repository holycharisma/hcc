use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::sea_orm::user_attributes::prelude::UserAttributes;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique, index)]
    pub email: String,

    #[sea_orm(unique)]
    pub username: String,
    pub password: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub active: bool
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserAttributes
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserAttributes => Entity::has_one(UserAttributes).into(),
        }
    }
}

impl Related<UserAttributes> for Entity {
    fn to() -> RelationDef {
        Relation::UserAttributes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}