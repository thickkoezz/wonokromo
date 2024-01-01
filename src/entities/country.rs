use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "country")]
#[graphql(complex)]
#[graphql(name = "Country")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::link_history::Entity")]
    LinkHistory,
    #[sea_orm(has_many = "super::user::Entity")]
    User,
}

impl Related<super::link_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkHistory.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
