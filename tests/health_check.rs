use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = urban_system::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

// Use the following command for checking the generated code
// cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=LeBron%20James&email=second_goat%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=LeBum", "missing the email"),
        ("email=something%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
