//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "owner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub crate_fk: i64,
    pub user_fk: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::krate::Entity",
        from = "Column::CrateFk",
        to = "super::krate::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Krate,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserFk",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::krate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Krate.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
