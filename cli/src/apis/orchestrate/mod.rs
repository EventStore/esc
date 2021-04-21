use structopt::StructOpt;
pub mod history;
pub mod jobs;

#[derive(StructOpt, Debug)]
#[structopt(about = "Orchestrates on going tasks, such as scheduled jobs")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    History(history::Command),
    Jobs(jobs::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::History(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Jobs(arg) => arg.command.exec(&cfg).await,
        }
    }
}
