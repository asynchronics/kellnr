use sea_orm_migration::prelude::*;

use crate::iden::{CrateIden, CrateUserIden, UserIden};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CrateUserIden::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CrateUserIden::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(CrateUserIden::CrateFk)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CrateUserIden::UserFk)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("crate_fk")
                            .from(CrateUserIden::Table, CrateUserIden::CrateFk)
                            .to(CrateIden::Table, CrateIden::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_fk")
                            .from(CrateUserIden::Table, CrateUserIden::UserFk)
                            .to(UserIden::Table, UserIden::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CrateUserIden::Table).to_owned())
            .await
    }
}
