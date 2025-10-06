#[derive(Serialize, Deserialize, FromRow)]
pub struct Image {
    pub idImage: i32,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}