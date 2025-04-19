

pub const DELETED: i8 = 1;
pub const NOT_DELETE: i8 = 0;

pub const LOCKED: i8 = 1;
pub const NOT_LOCK: i8 = 0;

pub mod functions {
    use diesel::sql_function;

    sql_function! {
        #[aggregate]
        #[sql_name = "last_insert_id"]
        fn last_insert_id_i32() -> Integer
    }
    sql_function!(fn last_insert_id() -> Bigint);
}

type DB = diesel::mysql::Mysql;

#[macro_export]
macro_rules! debug_sql {
    ( $x:expr ) => {{
        tracing::debug!(
            "Executing Query: {}",
            diesel::debug_query::<$crate::models::DB, _>($x).to_string()
        );
    }};
}
