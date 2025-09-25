mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{http, web};
use handlers::*;
use models::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

#[shuttle_runtime::main]
async fn main() -> shuttle_actix_web::ShuttleActixWeb<
    impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
> {
    // Cargar la base de datos
    let db = match load_db().await {
        Ok(db) => Arc::new(RwLock::new(db)),
        Err(e) => {
            error!("Error al cargar db_normalized.json: {:?}", e);
            return Err(shuttle_runtime::Error::Custom(e));
        }
    };

    let db_data = web::Data::new(db);

    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
            ])
            .max_age(3600);

        cfg.app_data(db_data.clone()).service(
            web::scope("")
                .wrap(cors)
                .service(get_areas)
                .service(get_area)
                .service(get_area_competencias)
                .service(get_competencia)
                .service(get_capacidad)
                .service(get_competencias)
                .service(get_capacidades_by_competencia),
        );
    };

    Ok(config.into())
}

async fn load_db() -> anyhow::Result<Database> {
    let file = include_str!("../db_normalized.json");
    let db: Database = serde_json::from_str(file)
        .map_err(|e| anyhow::anyhow!("db_normalized.json no es un JSON válido: {}", e))?;
    Ok(db)
}
