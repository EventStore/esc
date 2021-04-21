use structopt::StructOpt;
pub mod groups;
pub mod invites;
pub mod members;
pub mod policies;
pub mod settings;
pub mod tokens;

#[derive(StructOpt, Debug)]
#[structopt(about = "manages access control for the Event Store Cloud")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    Groups(groups::Command),
    Invites(invites::Command),
    Members(members::Command),
    Policies(policies::Command),
    Settings(settings::Command),
    Tokens(tokens::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::Groups(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Invites(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Members(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Policies(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Settings(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Tokens(arg) => arg.command.exec(&cfg).await,
        }
    }
}
