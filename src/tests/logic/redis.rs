use crate::domain::Details;
use crate::libraries::bb8_redis::redis::cmd;

#[ignore]
#[tokio::test]
async fn redis_connector_validation() {
    let datastore = Details::from_env("REDIS")
        .await
        .unwrap();
    let redis = datastore.redis().unwrap();
    let client = redis.get_client();
    let mut conn = client.get().await.unwrap();
    let _: () = cmd("PING").query_async(&mut *conn).await.unwrap();
}
