use super::config::TokenConfig;
use super::errors::IdentityError;
use super::tokens::Token;
use serde::de::DeserializeOwned;

pub type Result<T> = core::result::Result<T, IdentityError>;

async fn parse_result<T>(resp: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();
    if status.is_success() {
        let result: T = resp.json().await?;
        return Ok(result);
    }
    let message = resp.text().await?;
    Err(IdentityError {
        message,
        status_code: Some(status),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MfaRequiredError {
    error: String,
    mfa_token: Option<String>,
}

pub async fn create(
    client: &reqwest::Client,
    config: &TokenConfig,
    user_name: &str,
    password: &str,
) -> Result<Token> {
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "password");
    form.insert("username", user_name);
    form.insert("password", password);
    form.insert("scope", "cloud:access offline_access");
    form.insert("client_id", config.client_id.as_ref());
    form.insert("audience", &config.audience);

    let url = format!("{}/oauth/token", &config.identity_url);
    let req = client.post(url.as_str()).form(&form);

    let resp = req.send().await?;

    if resp.status().is_success() {
        let result: Token = resp.json().await?;
        Ok(result)
    } else {
        let mfa_token = get_mfa_token_or_error(resp).await?;
        challenge_mfa_and_confirm_otp(client, config, &mfa_token).await?;
        let otp = prompt_for_otp()?;
        create_with_otp(client, config, mfa_token, otp).await
    }
}

async fn get_mfa_token_or_error(resp: reqwest::Response) -> Result<String> {
    let status = resp.status();
    if status == 403 {
        let result: std::result::Result<MfaRequiredError, reqwest::Error> = resp.json().await;
        match result {
            Ok(error) => {
                if error.error == "mfa_required" {
                    match error.mfa_token {
                        None => {
                            Err(IdentityError {
                                    message: "Identity returned a 403 with an mfa_required error code, but no token.".to_string(),
                                    status_code: Some(status),
                                })
                        }
                        Some(mfa_token) => Ok(mfa_token),
                    }
                } else {
                    Err(IdentityError {
                        message: "not authorized".to_string(),
                        status_code: Some(status),
                    })
                }
            }
            Err(err) => {
                Err(IdentityError {
                        message: format!("Identity returned a 403 which could not be converted into a known error format: {}", err),
                        status_code: Some(status),
                    })
            }
        }
    } else {
        Err(IdentityError {
            message: "not authorized".to_string(),
            status_code: Some(status),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MfaChallengeArgs {
    mfa_token: String,
    challenge_type: String,
    client_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MfaChallengeResp {
    challenge_type: String,
}

async fn challenge_mfa_and_confirm_otp(
    client: &reqwest::Client,
    config: &TokenConfig,
    mfa_token: &str,
) -> Result<()> {
    let args = MfaChallengeArgs {
        challenge_type: "otp".to_string(),
        client_id: config.client_id.clone(),
        mfa_token: mfa_token.to_string(),
    };

    let url = format!("{}/mfa/challenge", &config.identity_url);
    let req = client.post(url.as_str()).json(&args);

    let resp = req.send().await?;
    let resp: MfaChallengeResp = parse_result(resp).await?;
    if resp.challenge_type == "otp" {
        Ok(())
    } else {
        Err(IdentityError {
            message: "Challenge type for this user's MFA was not OTP.".to_string(),
            status_code: None,
        })
    }
}

fn prompt_for_otp() -> Result<String> {
    let mut editor = rustyline::Editor::<()>::new();
    let result =
        editor.readline("Enter your one time password from your authenticator app or device: ");
    match result {
        Ok(line) => Ok(line),
        Err(err) => Err(IdentityError {
            message: format!("Error reading one time password: {}", err),
            status_code: None,
        }),
    }
}

pub async fn create_with_otp(
    client: &reqwest::Client,
    config: &TokenConfig,
    mfa_token: String,
    otp: String,
) -> Result<Token> {
    let mut form = std::collections::HashMap::new();

    form.insert("client_id", config.client_id.as_ref());
    form.insert("grant_type", "http://auth0.com/oauth/grant-type/mfa-otp");
    form.insert("mfa_token", mfa_token.as_ref());
    form.insert("otp", otp.as_ref());

    let url = format!("{}/oauth/token", &config.identity_url);
    let req = client.post(url.as_str()).form(&form);

    let resp = req.send().await?;

    parse_result(resp).await
}

pub async fn refresh(
    client: &reqwest::Client,
    config: &TokenConfig,
    refresh_token: &str,
) -> Result<Token> {
    let url = format!("{}/oauth/token", &config.identity_url);
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "refresh_token");
    form.insert("client_id", config.client_id.as_ref());
    form.insert("refresh_token", refresh_token);

    let req = client.post(url.as_str()).form(&form);

    debug!("Token refresh on : {:?}", req);

    let token: Token = parse_result(req.send().await?).await?;

    debug!("Token expires_in: {}", token.expires_in);

    Ok(token)
}
