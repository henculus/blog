use actix_web::web::Json;
pub use post::*;

mod post;

type JsonResponse<T, E> = Result<Json<T>, E>;
