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
    // Image,
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::Image => Entity::belongs_to(super::image::Entity)
//                 .from(Column::IdImage)
//                 .to(super::image::Column::IdImage)
//                 .into(),
//         }
//     }
// }

impl ActiveModelBehavior for ActiveModel {

}