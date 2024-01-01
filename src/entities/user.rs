use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user")]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub email: String,
    pub is_email_verified: bool,
    pub password: Option<String>,
    pub is_google_sso: bool,
    pub links_total: Option<i64>,
    pub tags_total: Option<i64>,
    pub created_at: DateTime,
    pub address: Option<String>,
    pub country_id: Option<i16>,
    pub salt: String,
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
    #[sea_orm(has_many = "super::custom_alias::Entity")]
    CustomAlias,
    #[sea_orm(has_many = "super::link::Entity")]
    Link,
    #[sea_orm(has_many = "super::link_tag::Entity")]
    LinkTag,
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<super::custom_alias::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomAlias.def()
    }
}

impl Related<super::link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Link.def()
    }
}

impl Related<super::link_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
