use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;

use crate::models::Database;

// GET /areas
#[get("/areas")]
pub async fn get_areas(db: web::Data<Arc<RwLock<Database>>>) -> impl Responder {
    let db = db.read().await;
    let areas: Vec<_> = db.areas.iter().map(|a| {
        json!({ "id": a.id, "nombre": a.nombre })
    }).collect();
    HttpResponse::Ok().json(areas)
}

// GET /areas/{id}
#[get("/areas/{id}")]
pub async fn get_area(
    db: web::Data<Arc<RwLock<Database>>>,
    id: web::Path<u32>,
) -> impl Responder {
    let db = db.read().await;
    let area = db.areas.iter().find(|a| a.id == *id);
    match area {
        Some(a) => HttpResponse::Ok().json(a),
        None => HttpResponse::NotFound().json(json!({ "error": "Área no encontrada" })),
    }
}

// GET /areas/{id}/competencias
#[get("/areas/{id}/competencias")]
pub async fn get_area_competencias(
    db: web::Data<Arc<RwLock<Database>>>,
    id: web::Path<u32>,
) -> impl Responder {
    let db = db.read().await;
    let area = db.areas.iter().find(|a| a.id == *id);
    match area {
        Some(a) => HttpResponse::Ok().json(&a.competencias),
        None => HttpResponse::NotFound().json(json!({ "error": "Área no encontrada" })),
    }
}

// GET /competencias/{id}
#[get("/competencias/{id}")]
pub async fn get_competencia(
    db: web::Data<Arc<RwLock<Database>>>,
    id: web::Path<u32>,
) -> impl Responder {
    let db = db.read().await;
    for area in &db.areas {
        if let Some(comp) = area.competencias.iter().find(|c| c.id == *id) {
            return HttpResponse::Ok().json(comp);
        }
    }
    HttpResponse::NotFound().json(json!({ "error": "Competencia no encontrada" }))
}

// GET /capacidades/{id}
#[get("/capacidades/{id}")]
pub async fn get_capacidad(
    db: web::Data<Arc<RwLock<Database>>>,
    id: web::Path<u32>,
) -> impl Responder {
    let db = db.read().await;
    for area in &db.areas {
        for comp in &area.competencias {
            if let Some(cap) = comp.capacidades.iter().find(|c| c.id == *id) {
                return HttpResponse::Ok().json(cap);
            }
        }
    }
    HttpResponse::NotFound().json(json!({ "error": "Capacidad no encontrada" }))
}


/// GET /competencias
/// Lista todas las competencias de todas las áreas
#[get("/competencias")]
pub async fn get_competencias(
    db: web::Data<Arc<RwLock<Database>>>
) -> impl Responder {
    let db = db.read().await;
    let mut competencias = Vec::new();
    for area in &db.areas {
        for comp in &area.competencias {
            competencias.push(json!({
                "id": comp.id,
                "nombre": comp.nombre,
                "area_id": area.id,
                "area_nombre": area.nombre
            }));
        }
    }
    HttpResponse::Ok().json(competencias)
}

/// GET /capacidades?competencia_id=101
/// Lista las capacidades de una competencia específica
#[get("/capacidades")]
pub async fn get_capacidades_by_competencia(
    db: web::Data<Arc<RwLock<Database>>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let db = db.read().await;

    if let Some(comp_id_str) = query.get("competencia_id") {
        if let Ok(comp_id) = comp_id_str.parse::<u32>() {
            for area in &db.areas {
                if let Some(comp) = area.competencias.iter().find(|c| c.id == comp_id) {
                    return HttpResponse::Ok().json(&comp.capacidades);
                }
            }
            return HttpResponse::NotFound().json(json!({ "error": "Competencia no encontrada" }));
        } else {
            return HttpResponse::BadRequest().json(json!({ "error": "competencia_id debe ser un número" }));
        }
    }

    HttpResponse::BadRequest().json(json!({
        "error": "Se requiere el parámetro competencia_id en la query"
    }))
}