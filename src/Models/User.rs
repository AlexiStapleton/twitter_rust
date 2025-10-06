#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct User {
    pub idUser: i32,
    pub username: String,
    pub hash_password: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub followers: i32,
    pub following: i32,
    pub posts: i32,
    pub likes: i32,
    pub comments: i32,
    pub created_at: String,
    pub updated_at: String,
}