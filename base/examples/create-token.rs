use esc_client_base::identity::client_id::ClientId;
use esc_client_base::identity::constants;
use esc_client_base::identity::operations;
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Usage: {} <email> <password>",
            args.get(0).expect("missing program name")
        );
    } else {
        let user_name = &args[1];
        let password = &args[2];
        println!("{} / {}", user_name, password);

        let client = Client::builder()
            .build()
            .expect("expected to build a client");

        let client_id = ClientId(constants::ES_CLIENT_ID.to_string());
        let result = operations::create(
            &client,
            constants::ES_CLOUD_IDENTITY_URL,
            &client_id,
            user_name,
            password,
            &constants::ES_CLOUD_API_AUDIENCE,
        )
        .await;

        match result {
            Ok(token) => {
                println!("token={}", token.access_token());
                println!("Authorization Header={}", token.authorization_header());
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
