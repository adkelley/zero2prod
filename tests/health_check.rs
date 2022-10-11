//! tests/health_check.rs

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed our spawn app.");
    let client = reqwest::Client::new();

    // Act
    let repsonse = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(repsonse.status().is_success());
    assert_eq!(Some(0), repsonse.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}
