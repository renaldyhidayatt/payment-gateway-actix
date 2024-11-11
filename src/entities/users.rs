//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(unique)]
    pub noc_transfer: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::saldo::Entity")]
    Saldo,
    #[sea_orm(has_many = "super::topups::Entity")]
    Topups,
    #[sea_orm(has_many = "super::withdraws::Entity")]
    Withdraws,
}

impl Related<super::saldo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Saldo.def()
    }
}

impl Related<super::topups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Topups.def()
    }
}

impl Related<super::withdraws::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Withdraws.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
