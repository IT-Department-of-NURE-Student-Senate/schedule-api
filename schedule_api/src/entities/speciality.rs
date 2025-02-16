//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "speciality")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub full_name: String,
    pub short_name: String,
    pub direction_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::direction::Entity",
        from = "Column::DirectionId",
        to = "super::direction::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Direction,
    #[sea_orm(has_many = "super::group::Entity")]
    Group,
}

impl Related<super::direction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Direction.def()
    }
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
