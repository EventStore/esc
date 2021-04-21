use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Tokens commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    ListUserTokens(ListUserTokens),
    DeleteUserToken(DeleteUserToken),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::ListUserTokens(params) => params.exec(cfg).await,
            CommandChoices::DeleteUserToken(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "fetchs a token user. TODO - this is probably not described correctly. Likely the security setting will need to change here to work correctly."
)]
pub struct ListUserTokens {}

impl ListUserTokens {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Tokens::new(sender)
            .list_user_tokens()
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "deletes a token TODO - this likely needs to change the security stuff as well"
)]
pub struct DeleteUserToken {
    #[structopt(long, help = "the id of the token")]
    pub token_id: String,
}

impl DeleteUserToken {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Tokens::new(sender)
            .delete_user_token(&self.token_id)
            .await?;

        Ok(())
    }
}
