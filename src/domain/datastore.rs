pub enum Datastore {
    #[cfg(feature = "sqlite")]
    SQLite(crate::infrastructure::sqlite::Connector),
    #[cfg(feature = "mysql")]
    MySQL(crate::infrastructure::mysql::Connector),
    #[cfg(feature = "postgres")]
    Postgres(crate::infrastructure::postgres::Connector),
    #[cfg(feature = "redis")]
    Redis(crate::infrastructure::redis::Connector),
    #[cfg(feature = "surreal")]
    Surreal(crate::infrastructure::surrealdb::Connector),
    Disabled,
}
