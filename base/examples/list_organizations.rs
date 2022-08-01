// This example just shows how to put the Client together
use std::env;
use std::sync::Arc;

use esc_client_base::client::Authorization;
use esc_client_base::client::Client;
use esc_client_base::requests::RequestSender;

pub struct StaticAuthorization {
    pub authorization_header: String,
}

impl Authorization for StaticAuthorization {
    fn authorization_header(&self) -> String {
        self.authorization_header.clone()
    }

    fn refresh(&mut self) -> bool {
        false
    }
}

pub fn static_token_client(base_url: String, authorization_header: String) -> Client {
    let authorization = Arc::new(StaticAuthorization {
        authorization_header,
    });
    let sender = RequestSender {
        client: reqwest::Client::new(),
        observer: None,
    };
    Client {
        authorization,
        base_url,
        sender,
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Usage: {} <base-url> <authorization-header>",
            args.get(0).expect("missing program name")
        );
    } else {
        let base_url = args.get(1).expect("missing base url");
        let authorization_header = args.get(2).expect("missing auth header");
        let _client = static_token_client(base_url.clone(), authorization_header.clone());
        // let resp = client.send_request<(), crate::resources::models::ListOrganizationsResponse>(reqwest::Method::GET, "resources/v1/organizations", None, true);
    }
}
