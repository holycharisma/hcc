use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::sea_orm::user::prelude::{User, UserColumn};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_attr")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub user_id: i32,

    pub send_broadcast_email: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(User)
                .from(Column::UserId)
                .to(UserColumn::Id)
                .into(),
        }
    }
}

impl Related<User> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {

}