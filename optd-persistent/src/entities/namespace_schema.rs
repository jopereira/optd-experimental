//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "namespace_schema")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub database_id: i32,
    pub name: String,
    pub created_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::database_metadata::Entity",
        from = "Column::DatabaseId",
        to = "super::database_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    DatabaseMetadata,
    #[sea_orm(has_many = "super::table_metadata::Entity")]
    TableMetadata,
}

impl Related<super::database_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DatabaseMetadata.def()
    }
}

impl Related<super::table_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TableMetadata.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
