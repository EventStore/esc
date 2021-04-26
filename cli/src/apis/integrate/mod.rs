use structopt::StructOpt;
pub mod integrations;

#[derive(StructOpt, Debug)]
#[structopt(about = "Manages third-party integrations with the Event Store Cloud.")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    Integrations(integrations::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::Integrations(arg) => arg.command.exec(&cfg).await,
        }
    }
}
