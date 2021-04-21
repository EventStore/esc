use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Peerings commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListPeerings),
    Create(CreatePeering),
    CreatePeeringCommands(CreatePeeringCommands),
    Delete(DeletePeering),
    Get(GetPeering),
    Update(UpdatePeering),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::CreatePeeringCommands(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "list all peerings")]
pub struct ListPeerings {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListPeerings {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Peerings::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a peering request")]
pub struct CreatePeering {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long)]
    pub network_id: String,
    #[structopt(long)]
    pub description: String,
    #[structopt(long)]
    pub peer_account_id: String,
    #[structopt(long)]
    pub peer_network_id: String,
    #[structopt(long)]
    pub peer_network_region: String,
    #[structopt(long)]
    pub routes: Vec<String>,
}

impl CreatePeering {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Peerings::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::infra::CreatePeeringRequest {
                    network_id: self.network_id.clone(),
                    description: self.description.clone(),
                    peer_account_id: self.peer_account_id.clone(),
                    peer_network_id: self.peer_network_id.clone(),
                    peer_network_region: self.peer_network_region.clone(),
                    routes: self.routes.iter().map(|e| e.clone()).collect(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates peerings commands")]
pub struct CreatePeeringCommands {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long)]
    pub provider: String,
    #[structopt(long)]
    pub peer_account_id: String,
    #[structopt(long)]
    pub peer_network_id: String,
}

impl CreatePeeringCommands {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Peerings::new(sender)
            .create_commands(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::infra::CreatePeeringCommandsRequest {
                    provider: self.provider.clone(),
                    peer_account_id: self.peer_account_id.clone(),
                    peer_network_id: self.peer_network_id.clone(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a peering")]
pub struct DeletePeering {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the peering")]
    pub peering_id: String,
}

impl DeletePeering {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::infra::paths::Peerings::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::PeeringId(self.peering_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "retrieves a peering")]
pub struct GetPeering {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the peering")]
    pub peering_id: String,
}

impl GetPeering {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::infra::paths::Peerings::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::PeeringId(self.peering_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates a peering")]
pub struct UpdatePeering {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the peering")]
    pub peering_id: String,
    #[structopt(long)]
    pub description: String,
}

impl UpdatePeering {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::infra::paths::Peerings::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::PeeringId(self.peering_id.clone()),
                esc_api::infra::UpdatePeeringRequest {
                    description: self.description.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
