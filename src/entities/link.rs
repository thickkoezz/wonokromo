use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "link")]
#[graphql(complex)]
#[graphql(name = "Link")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub short_url: String,
    pub original_url: String,
    pub user_id: Option<i64>,
    pub created_at: DateTime,
    pub total_hit: i32,
    pub average_hit_per_month: i32,
    pub average_hit_per_week: i32,
    pub max_hit_in_a_month: i32,
    pub min_hit_in_a_month: i32,
    pub qr_code: Option<String>,
    pub is_custom_alias: bool,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "super::link_history::Entity")]
    LinkHistory,
    #[sea_orm(has_many = "super::link_timeline::Entity")]
    LinkTimeline,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::link_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkHistory.def()
    }
}

impl Related<super::link_timeline::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkTimeline.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
