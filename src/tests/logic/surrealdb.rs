use crate::domain::Details;

#[ignore]
#[tokio::test]
async fn surreal_connector_validation() {
    let datastore = Details::from_env("SURREALDB")
        .await
        .unwrap();
    let surrealdb = datastore.surrealdb().unwrap();
    surrealdb.get_client().health().await.unwrap();
}
