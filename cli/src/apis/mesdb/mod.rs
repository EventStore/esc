use structopt::StructOpt;
pub mod backups;
pub mod clusters;

#[derive(StructOpt, Debug)]
#[structopt(about = "Works with Event Store databases")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    Backups(backups::Command),
    Clusters(clusters::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::Backups(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Clusters(arg) => arg.command.exec(&cfg).await,
        }
    }
}
