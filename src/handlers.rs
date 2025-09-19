use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{Database};

// GET /areas
#[get("/areas")]
pub async fn get_areas(db: web::Data<Arc<RwLock<Database>>>) -> impl Responder {
    let db = db.read().await;
    let areas: Vec<&str> = db.areas.iter().map(|a| a.nombre.as_str()).collect();
    HttpResponse::Ok().json(areas)
}

// GET /areas/{nombre}
#[get("/areas/{nombre}")]
pub async fn get_area(
    db: web::Data<Arc<RwLock<Database>>>,
    nombre: web::Path<String>,
) -> impl Responder {
    let db = db.read().await;
    let area = db.areas.iter().find(|a| a.nombre.eq_ignore_ascii_case(&nombre));
    match area {
        Some(a) => HttpResponse::Ok().json(a),
        None => HttpResponse::NotFound().body("Área no encontrada"),
    }
}

// GET /areas/{nombre}/competencias
#[get("/areas/{nombre}/competencias")]
pub async fn get_area_competencias(
    db: web::Data<Arc<RwLock<Database>>>,
    nombre: web::Path<String>,
) -> impl Responder {
    let db = db.read().await;
    let area = db.areas.iter().find(|a| a.nombre.eq_ignore_ascii_case(&nombre));
    match area {
        Some(a) => HttpResponse::Ok().json(&a.competencias),
        None => HttpResponse::NotFound().body("Área no encontrada"),
    }
}

// GET /competencias/{nombre}
#[get("/competencias/{nombre}")]
pub async fn get_competencia(
    db: web::Data<Arc<RwLock<Database>>>,
    nombre: web::Path<String>,
) -> impl Responder {
    let db = db.read().await;
    for area in &db.areas {
        if let Some(comp) = area.competencias.iter().find(|c| c.nombre.eq_ignore_ascii_case(&nombre)) {
            return HttpResponse::Ok().json(comp);
        }
    }
    HttpResponse::NotFound().body("Competencia no encontrada")
}

// GET /capacidades/{nombre}
#[get("/capacidades/{nombre}")]
pub async fn get_capacidad(
    db: web::Data<Arc<RwLock<Database>>>,
    nombre: web::Path<String>,
) -> impl Responder {
    let db = db.read().await;
    for area in &db.areas {
        for comp in &area.competencias {
            if let Some(cap) = comp.capacidades.iter().find(|c| c.nombre.eq_ignore_ascii_case(&nombre)) {
                return HttpResponse::Ok().json(cap);
            }
        }
    }
    HttpResponse::NotFound().body("Capacidad no encontrada")
}