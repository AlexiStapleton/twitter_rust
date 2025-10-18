use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    // Essaie de lire la variable d'environnement DATABASE_URL
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/twitter".to_string());

    // Tente la connexion à la base
    Database::connect(&database_url)
        .await
        .expect("❌ Impossible de se connecter à la base de données")
}