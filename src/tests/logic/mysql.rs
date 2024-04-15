use crate::domain::Details;
use crate::libraries::sqlx::query;

#[tokio::test]
async fn mysql_connector_validation() {
    let datastore = Details::from_env("MYSQL")
        .await
        .unwrap();
    let mysql = datastore.mysql().unwrap();
    let _ = query("SELECT 1;")
        .fetch_one(mysql.get_client())
        .await
        .unwrap();
}
