#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub idPost: i32,
    pub idUser: i32,
    pub idImage: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}