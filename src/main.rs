mod models;
use models::user::{ActiveModel as UserModel, Entity as User};
use sea_orm::{Database, Set};
use crate::models::user::ActiveModel;

#[tokio::main]
async fn main(){
    let connection = Database::connect("postgres://postgres:postgres@localhost:5432/twitter").await.unwrap();

    // let user1: ActiveModel = UserModel{
    //     username: Set("bojodo".to_string()),
    //     hash_password: Set("kjhsdhgfsdfksdhfk".to_string()),
    //     email: Set
    // };
}