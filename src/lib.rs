pub mod libraries;

mod builders {
    mod details;
    pub use details::Builder as DetailsBuilder;
}

pub mod domain {
    mod errors;
    pub use errors::DatastoreConnectError;
    mod datastore;
    pub use datastore::Datastore;

    mod details;
    pub use details::Details;
}

mod infrastructure {
    #[cfg(feature = "mysql")]
    pub mod mysql;
    #[cfg(feature = "postgres")]
    pub use mysql::Connector as MySQLConnector;

    #[cfg(feature = "postgres")]
    pub mod postgres;
    #[cfg(feature = "postgres")]
    pub use postgres::Connector as PostgresConnector;


    #[cfg(feature = "redis")]
    pub mod redis;
    #[cfg(feature = "redis")]
    pub use redis::Connector as RedisConnector;

    #[cfg(feature = "sqlite")]
    pub mod sqlite;
    #[cfg(feature = "sqlite")]
    pub use sqlite::Connector as SQLiteConnector;

    #[cfg(feature = "surreal")]
    pub mod surrealdb;
    #[cfg(feature = "surreal")]
    pub use surrealdb::Connector as SurrealConnector;
}

pub mod logic {
    mod details;
    mod datastore;
}

#[cfg(test)]
mod tests {
    mod builders {
        mod details;
    }
    mod logic {

        #[cfg(feature = "mysql")]
        mod mysql;

        #[cfg(feature = "postgres")]
        mod postgres;

        #[cfg(feature = "redis")]
        mod redis;

        #[cfg(feature = "sqlite")]
        mod sqlite;

        #[cfg(feature = "surreal")]
        mod surrealdb;
    }
}
