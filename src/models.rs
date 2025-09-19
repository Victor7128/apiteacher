use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capacidad {
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competencia {
    pub nombre: String,
    pub descripcion: String,
    pub capacidades: Vec<Capacidad>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub nombre: String,
    pub competencias: Vec<Competencia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub areas: Vec<Area>,
}