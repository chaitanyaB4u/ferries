pub mod prelude {

    use diesel::prelude::*;

    fn get_test_database_url() -> String {
        dotenv::dotenv().ok();
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL environment variable should be test.")
    }

    pub fn connection_without_transaction() -> MysqlConnection {
        let db_url = get_test_database_url();
        MysqlConnection::establish(db_url.as_str()).unwrap()
    }
}

pub mod authentication_feature;
pub mod registration_feature;
pub mod password_reset_feature;

pub mod session_tests;
