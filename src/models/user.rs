use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_user: i32,
    pub id_image: Option<i32>,
    pub username: String,
    pub hash_password: String,
    pub email: String,
    pub bio: Option< String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Posts,
    #[sea_orm(has_many = "super::like::Entity")]
    Likes,
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::IdImage",
        to = "super::image::Column::IdImage"
    )]
    Image,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<super::like::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl ActiveModelBehavior for ActiveModel {

}