use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main(
) -> ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static> {
}
