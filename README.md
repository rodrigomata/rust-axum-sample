# Rust Axum Example
Basic HTTP server that exposes a health check endpoint to learn how to do that in Rust. 

It decouples the main function into its own function so that it can be parameterized and easier to test. The example contains one integration test and adds git hooks to format on commit and test on push.