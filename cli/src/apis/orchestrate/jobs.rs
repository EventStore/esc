use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Jobs commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListJobs),
    Create(CreateJob),
    Delete(DeleteJob),
    Get(GetJob),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "returns the list of jobs under a project")]
pub struct ListJobs {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListJobs {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::orchestrate::paths::Jobs::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "creates a new job")]
pub struct CreateJob {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(subcommand)]
    pub data: JobData,
    #[structopt(long)]
    pub description: String,
    #[structopt(long)]
    pub schedule: String,
}

impl CreateJob {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::orchestrate::paths::Jobs::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::orchestrate::CreateJobRequest {
                    data: match &self.data {
                        JobData::ScheduledBackup(args) => {
                            esc_api::orchestrate::JobData::ScheduledBackup(
                                esc_api::orchestrate::ScheduledBackupData {
                                    cluster_id: esc_api::ClusterId(args.cluster_id.clone()),
                                    description: args.description.clone(),
                                    max_backup_count: args.max_backup_count.clone(),
                                },
                            )
                        }
                    },
                    description: self.description.clone(),
                    schedule: self.schedule.clone(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "delete a job")]
pub struct DeleteJob {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "the id of the job")]
    pub job_id: String,
}

impl DeleteJob {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::orchestrate::paths::Jobs::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::JobId(self.job_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a job")]
pub struct GetJob {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the job")]
    pub job_id: String,
}

impl GetJob {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::orchestrate::paths::Jobs::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::JobId(self.job_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
pub enum JobData {
    ScheduledBackup(ScheduledBackupData),
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct ScheduledBackupData {
    #[structopt(long)]
    pub cluster_id: String,
    #[structopt(long)]
    pub description: String,
    #[structopt(long)]
    pub max_backup_count: i32,
}
