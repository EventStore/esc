use esc_client_base::errors::ApiResponseError;
use esc_client_base::errors::ProblemDetails;
use esc_client_base::errors::Result;

#[allow(clippy::result_large_err)]
fn get_api_error() -> std::result::Result<i32, ApiResponseError> {
    let status_code = reqwest::StatusCode::INTERNAL_SERVER_ERROR;
    let problem_details = ProblemDetails {
        detail: Some("Details Here".to_string()),
        fields: Default::default(),
        instance: "Instance".to_string(),
        status: 200,
        title: "Title".to_string(),
        _type: "Type".to_string(),
    };
    let err = ApiResponseError {
        problem_details,
        status_code,
    };
    Err(err)
}

#[allow(clippy::result_large_err)]
fn return_error() -> Result<i32> {
    get_api_error()?;
    panic!("get_api_error should have failed");
}

#[allow(clippy::result_large_err)]
fn expect_error() -> Result<u16> {
    let result = return_error();
    match result {
        Ok(_) => panic!("function should have error'd"),
        Err(err) => {
            let resp = err.api_response()?;
            Ok(resp.status_code.as_u16())
        }
    }
}

async fn some_async_function() {
    use std::time::Duration;
    use tokio::time::sleep;
    sleep(Duration::new(10, 0)).await;
}

async fn async_expect_error() -> Result<u16> {
    let result = return_error();
    some_async_function().await;
    match result {
        Ok(_) => panic!("function should have error'd"),
        Err(err) => {
            let resp = err.api_response()?;
            Ok(resp.status_code.as_u16())
        }
    }
}

#[test]
fn test_checking_api_response_from_error() {
    assert_eq!(Ok(500u16), expect_error());
}

#[tokio::test]
async fn test_async_function_error_handling() {
    assert_eq!(Ok(500u16), async_expect_error().await);
}
