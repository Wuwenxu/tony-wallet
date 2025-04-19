use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, post, put};
use crate::bootstrap::result;
use crate::bootstrap::response::Response;

#[get("")]
pub async fn list(
) -> result::Response {

    Response::success((""))
}