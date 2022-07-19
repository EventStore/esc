use crate::identity::Token;
use crate::sender::RequestSender;
use super::client::Client;
use super::authorization::StaticTokenAuthorizer;

fn static_token_client(token: Token) -> Client {
    let authorizer = StaticTokenAut
}