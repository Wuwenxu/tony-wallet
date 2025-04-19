use actix_web::web;

use crate::app::admin::*;
use crate::routes::admin;

pub fn register_routes(s: &mut web::ServiceConfig) {
    s.service(
        // /api/v1/
        web::scope("/v1")
            // /api/v1/adminTest
            .service(
                web::scope("/adminTest")
                    .service(admin_test::list)
            )
    );
}
