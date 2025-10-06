use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_post: i32,
    pub id_user: i32,
    pub id_image: Option<i32>,
    pub parent_id: Option<i32>,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::IdUser",
        to = "super::user::Column::IdUser",
    )]
    User,

    #[sea_orm(has_many = "super::like::Entity")]
    Likes,

    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::IdPost",
    )]
    Father,
    
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::IdImage",
        to = "super::image::Column::IdImage",
    )]
    Image
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::like::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Father.def()
    }
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl ActiveModelBehavior for ActiveModel {

}