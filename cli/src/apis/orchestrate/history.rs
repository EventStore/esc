use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers History commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListHistory),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "shows orchestration history")]
pub struct ListHistory {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "Filter by job id")]
    pub job_id: Option<String>,
}

impl ListHistory {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::orchestrate::paths::History::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                self.job_id.clone(),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}
