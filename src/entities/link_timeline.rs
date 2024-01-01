use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "link_timeline")]
#[graphql(complex)]
#[graphql(name = "LinkTimeline")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub short_url: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub period: Date,
    pub hits_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::link::Entity",
        from = "Column::ShortUrl",
        to = "super::link::Column::ShortUrl",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Link,
}

impl Related<super::link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Link.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
