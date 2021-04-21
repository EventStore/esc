use structopt::StructOpt;
pub mod organizations;
pub mod projects;

#[derive(StructOpt, Debug)]
#[structopt(about = "APIs used to access customer defined resources")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(StructOpt, Debug)]
pub enum CommandChoices {
    Organizations(organizations::Command),
    Projects(projects::Command),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::Organizations(arg) => arg.command.exec(&cfg).await,
            CommandChoices::Projects(arg) => arg.command.exec(&cfg).await,
        }
    }
}
