#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::country")]
#[seaography(entity = "crate::entities::custom_alias")]
#[seaography(entity = "crate::entities::link")]
#[seaography(entity = "crate::entities::link_history")]
#[seaography(entity = "crate::entities::link_tag")]
#[seaography(entity = "crate::entities::link_timeline")]
#[seaography(entity = "crate::entities::user")]
pub struct QueryRoot;
