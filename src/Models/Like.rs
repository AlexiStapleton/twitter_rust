use sea_orm::entity::prelude::*;
#[derive(Serialize, Deserialize, FromRow)]
pub struct Like {
    pub idLike: i32,
    pub idUser: i32,
    pub idPost: i32,
    pub created_at: String,
    pub updated_at: String,
}