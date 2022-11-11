mod tests {
    use axum::{http::{Request, StatusCode}, body::Body};
    use axum_sample_lib::router;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_check_works() {
        let app = router();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
