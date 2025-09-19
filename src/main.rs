mod handlers;
mod models;

use actix_web::web;
use handlers::*;
use models::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

#[shuttle_runtime::main]
async fn main() -> shuttle_actix_web::ShuttleActixWeb<
    impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
> {
    let db = Arc::new(RwLock::new(
        load_db().await.expect("No se pudo cargar la base de datos"),
    ));
    let db_data = web::Data::from(db);

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(db_data.clone())
            .service(get_areas)
            .service(get_area)
            .service(get_area_competencias)
            .service(get_competencia)
            .service(get_capacidad);
    };

    Ok(config.into())
}

async fn load_db() -> anyhow::Result<Database> {
    let file = tokio::fs::read_to_string("db.json").await?;
    let db: Database = serde_json::from_str(&file)?;
    Ok(db)
}
