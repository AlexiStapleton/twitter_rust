use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_image: i32,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::IdImage",
        to = "super::post::Column::IdImage"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::IdImage",
        to = "super::user::Column::IdImage"
    )]
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