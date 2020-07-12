#[cfg(test)]
use super::*;
use actix_web::test;

#[actix_rt::test]
async fn test_index_ok() {
    let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
}

