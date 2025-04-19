pub mod app;
pub mod bootstrap;
pub mod config;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;


pub mod constant {

    pub const DEFAULT_PER_PAGE: i64 = 10;
    pub const DEFAULT_PAGE_NUM: i64 = 1;
    pub const DEFAULT_SUCCESS_MESSAGE: &str = "SUCCESS";
    pub const DEFAULT_FAIL_MESSAGE: &str = "FAIL";
}