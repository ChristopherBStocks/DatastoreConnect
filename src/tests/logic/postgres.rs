use crate::domain::Details;
use crate::libraries::sqlx::query;

#[tokio::test]
async fn postgres_connector_validation() {
    let datastore = Details::from_env("POSTGRES")
        .await
        .unwrap();
    let postgres = datastore.postgres().unwrap();
    let _ = query("SELECT 1;")
        .fetch_one(postgres.get_client())
        .await
        .unwrap();
}
