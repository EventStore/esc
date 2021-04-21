use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Backups commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListBackups),
    Create(CreateBackup),
    Delete(DeleteBackup),
    Get(GetBackup),
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
#[structopt(about = "List backups")]
pub struct ListBackups {
    #[structopt(long, help="The id of the organization the backup is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the backup is grouped under",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListBackups {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Backups::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new backup")]
pub struct CreateBackup {
    #[structopt(long, help="The id of the organization the backup is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the backup is grouped under",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long)]
    pub description: String,
    #[structopt(long)]
    pub source_cluster_id: String,
}

impl CreateBackup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Backups::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::mesdb::CreateBackupRequest {
                    description: self.description.clone(),
                    source_cluster_id: esc_api::ClusterId(self.source_cluster_id.clone()),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a backup")]
pub struct DeleteBackup {
    #[structopt(long, help="The id of the organization the backup is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the backup is grouped under",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the backup to retrieve")]
    pub backup_id: String,
}

impl DeleteBackup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Backups::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::BackupId(self.backup_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "get a single backup")]
pub struct GetBackup {
    #[structopt(long, help="The id of the organization the backup is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the backup is grouped under",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the backup to retrieve")]
    pub backup_id: String,
}

impl GetBackup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Backups::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::BackupId(self.backup_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}
