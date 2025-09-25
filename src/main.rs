mod handlers;
mod models;

use actix_web::web;
use handlers::*;
use models::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

#[shuttle_runtime::main]
async fn main() -> shuttle_actix_web::ShuttleActixWeb<
    impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
> {
    let db = match load_db().await {
        Ok(db) => Arc::new(RwLock::new(db)),
        Err(e) => {
            error!("Error al cargar db_normalized.json: {:?}", e);
            return Err(shuttle_runtime::Error::Custom(e));
        }
    };

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
    let file = tokio::fs::read_to_string("db_normalized.json")
        .await
        .map_err(|e| anyhow::anyhow!("No se pudo leer db_normalized.json: {}", e))?;
    let db: Database = serde_json::from_str(&file)
        .map_err(|e| anyhow::anyhow!("db_normalized.json no es un JSON válido: {}", e))?;
    Ok(db)
}
