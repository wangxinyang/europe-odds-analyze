use std::path::Path;

use sqlx_mock::TestPostgres;

pub struct TestConfig {
    pub tps: TestPostgres,
}

impl TestConfig {
    pub async fn new() -> Self {
        let tps = TestPostgres::new(
            "postgres://postgres:postgres@localhost:5432".into(),
            Path::new("../migrations"),
        );
        Self { tps }
    }
}
