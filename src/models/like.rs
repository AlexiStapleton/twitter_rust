use sea_orm::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "likes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_like: i32,
    pub id_user: i32,
    pub id_post: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::IdPost",
        to = "super::post::Column::IdPost" )]
    Post,
    #[sea_orm(
        belongs_to = "super::user::Entity", 
        from = "Column::IdUser", 
        to = "super::user::Column::IdUser")]
    User,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {

}