use sea_orm::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "likes")]
pub struct Model {
    #[sea_orm(primary_key)]   
    pub idLike: i32,
    pub idUser: i32,
    pub idPost: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {

}