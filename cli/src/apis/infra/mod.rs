use structopt::StructOpt;
pub mod networks;
pub mod peerings;

#[derive(StructOpt, Debug)]
#[structopt(about = "Creates infrastructure needed to work with Event Store DB instances")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    Networks(networks::Command),
    Peerings(peerings::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::Networks(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Peerings(arg) => arg.command.exec(&cfg).await,
        }
    }
}
