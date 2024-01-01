use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "link_history")]
#[graphql(complex)]
#[graphql(name = "LinkHistory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub short_url: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub time: DateTimeWithTimeZone,
    pub country_id: Option<i16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::country::Entity",
        from = "Column::CountryId",
        to = "super::country::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Country,
    #[sea_orm(
        belongs_to = "super::link::Entity",
        from = "Column::ShortUrl",
        to = "super::link::Column::ShortUrl",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Link,
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<super::link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Link.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
