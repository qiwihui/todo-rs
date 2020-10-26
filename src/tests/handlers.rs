use crate::tests::helpers::assert_get;

#[actix_rt::test]
async fn test_hello_world() {
    assert_get("/").await;
}
