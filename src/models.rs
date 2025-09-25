use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Database {
    pub areas: Vec<Area>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Area {
    pub id: u32,
    pub nombre: String,
    pub competencias: Vec<Competencia>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Competencia {
    pub id: u32,
    pub nombre: String,
    pub descripcion: String,
    pub capacidades: Vec<Capacidad>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Capacidad {
    pub id: u32,
    pub nombre: String,
    pub descripcion: String,
}