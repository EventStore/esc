use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Networks commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListNetworks),
    Create(CreateNetwork),
    Delete(DeleteNetwork),
    Get(GetNetwork),
    Update(UpdateNetwork),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "lists all networks under the given project")]
pub struct ListNetworks {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListNetworks {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Networks::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "creates a new network")]
pub struct CreateNetwork {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long)]
    pub provider: String,
    #[structopt(long)]
    pub cidr_block: String,
    #[structopt(long)]
    pub description: String,
    #[structopt(long)]
    pub region: String,
}

impl CreateNetwork {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Networks::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::infra::CreateNetworkRequest {
                    provider: self.provider.clone(),
                    cidr_block: self.cidr_block.clone(),
                    description: self.description.clone(),
                    region: self.region.clone(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a network")]
pub struct DeleteNetwork {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the network")]
    pub network_id: String,
}

impl DeleteNetwork {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::infra::paths::Networks::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::NetworkId(self.network_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a single network")]
pub struct GetNetwork {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the network")]
    pub network_id: String,
}

impl GetNetwork {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Networks::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::NetworkId(self.network_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates the given network")]
pub struct UpdateNetwork {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the network")]
    pub network_id: String,
    #[structopt(long)]
    pub description: String,
}

impl UpdateNetwork {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::infra::paths::Networks::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::NetworkId(self.network_id.clone()),
                esc_api::infra::UpdateNetworkRequest {
                    description: self.description.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
