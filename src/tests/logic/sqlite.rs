use crate::domain::Details;
use crate::libraries::sqlx::query;

#[tokio::test]
async fn sqlite_memory_connector_validation() {
    let datastore = Details::from_env("SQLITE")
        .await
        .unwrap();
    let sqlite = datastore.sqlite().unwrap();
    let _ = query("SELECT 1;")
        .fetch_one(sqlite.get_client())
        .await
        .unwrap();
}

#[tokio::test]
async fn sqlite_file_connector_validation() {
    let datastore = Details::from_env("SQLITE")
        .await
        .unwrap();
    let sqlite = datastore.sqlite().unwrap();
    let _ = query("SELECT 1;")
        .fetch_one(sqlite.get_client())
        .await
        .unwrap();
}
