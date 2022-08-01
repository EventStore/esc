use esc_api_base::errors::*;

fn get_api_error() -> std::result::Result<i32, ApiResponseError> {
    let status_code = reqwest::StatusCode::INTERNAL_SERVER_ERROR;
    let problem_details = ProblemDetails {
        details: "Details Here".to_string(),
        fields: Default::default(),
        instance: "Instance".to_string(),
        status: "Status".to_string(),
        title: "Title".to_string(),
        _type: "Type".to_string(),
    };
    let err = ApiResponseError {
        problem_details,
        status_code,
    };
    Err(err)
}

fn return_error() -> Result<i32> {
    get_api_error()?;
    panic!("get_api_error should have failed");
}

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

async fn some_async_function() -> () {
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
