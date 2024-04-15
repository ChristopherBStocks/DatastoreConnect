use crate::builders::DetailsBuilder;
use crate::domain::Details;

#[tokio::test]
pub async fn details_env_test() {
    let details: Details = DetailsBuilder::from_env("DATASTORE").build();
    assert_eq!(details.user, "root");
}

#[tokio::test]
pub async fn details_default_test() {
    let details: Details = DetailsBuilder::default().build();
    assert_eq!(details.user, "zephyr");
}