use domain::sea_orm::user_attributes::prelude::*;
use domain::sea_orm::user::prelude::{User, UserColumn};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m01_000003_create_user_attributes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let res = manager
            .create_table(
                Table::create()
                    .table(UserAttributes)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAttributesColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAttributesColumn::UserId)
                            .not_null()
                           .unique_key()
                            .integer(),
                    )
                    .col(
                        ColumnDef::new(UserAttributesColumn::SendBroadcastEmail)
                            .boolean(),
                    )
                    .to_owned(),

                   
            )
            .await;

        if res.is_ok() {

            manager.create_foreign_key(
                ForeignKey::create()
                .name("user_auth_has_user_attr")
                .from(UserAttributes, UserAttributesColumn::UserId)
                .to(User, UserColumn::Id)
                .to_owned()
            )
            .await

        } else {
            res
        }

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(UserAttributes)
                    .to_owned(),
            )
            .await
    }
}
