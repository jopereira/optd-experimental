//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "physical_property")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub physical_expression_id: i32,
    pub variant_tag: i16,
    pub data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::physical_expression::Entity",
        from = "Column::PhysicalExpressionId",
        to = "super::physical_expression::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PhysicalExpression,
}

impl Related<super::physical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhysicalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
