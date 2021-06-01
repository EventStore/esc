use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Integrations commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListIntegrations),
    Create(CreateIntegration),
    Delete(DeleteIntegration),
    Get(GetIntegration),
    Update(UpdateIntegration),
    TestIntegration(TestIntegration),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
            CommandChoices::TestIntegration(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "list all integrations")]
pub struct ListIntegrations {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListIntegrations {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrate::paths::Integrations::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new integration")]
pub struct CreateIntegration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(subcommand)]
    pub data: CreateIntegrationData,
    #[structopt(long)]
    pub description: String,
}

impl CreateIntegration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrate::paths::Integrations::new(sender).create(
            esc_api::OrgId(self.organization_id.clone()),
            esc_api::ProjectId(self.project_id.clone()),
        esc_api::integrate::CreateIntegrationRequest{
data: match &self.data {
        CreateIntegrationData::opsGenie(args) => esc_api::integrate::CreateIntegrationData::CreateOpsGenieIntegrationData{
            api_key: args.api_key.clone(),
            source: args.source.clone(),
},
        CreateIntegrationData::slack(args) => esc_api::integrate::CreateIntegrationData::CreateSlackIntegrationData{
            channel_id: args.channel_id.clone(),
            token: args.token.clone(),
            source: args.source.clone(),
},
},
            description: self.description.clone(),
            }
        ).await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a integration")]
pub struct DeleteIntegration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the integration")]
    pub integration_id: String,
}

impl DeleteIntegration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::integrate::paths::Integrations::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::IntegrationId(self.integration_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "retrieves a integration")]
pub struct GetIntegration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the integration")]
    pub integration_id: String,
}

impl GetIntegration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrate::paths::Integrations::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::IntegrationId(self.integration_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates a integration")]
pub struct UpdateIntegration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the integration")]
    pub integration_id: String,
    #[structopt(subcommand)]
    pub data: Option<UpdateIntegrationData>,
    #[structopt(long)]
    pub description: Option<String>,
}

impl UpdateIntegration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::integrate::paths::Integrations::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::IntegrationId(self.integration_id.clone()),
                esc_api::integrate::UpdateIntegrationRequest {
                    data: match &self.data {
                        None => None,
                        Some(data) => Some(esc_api::integrate::UpdateIntegrationData {
                            api_key: data.api_key.clone(),
                            channel_id: data.channel_id.clone(),
                            token: data.token.clone(),
                        }),
                    },
                    description: self.description.clone(),
                },
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Sends a message to an integration sink")]
pub struct TestIntegration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the integration")]
    pub integration_id: String,
}

impl TestIntegration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrate::paths::Integrations::new(sender)
            .test_integration(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::IntegrationId(self.integration_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
pub enum CreateIntegrationData {
    opsGenie(CreateOpsGenieIntegrationData),
    slack(CreateSlackIntegrationData),
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct CreateOpsGenieIntegrationData {
    #[structopt(long, help = "API key used with the Ops Genie integration API")]
    pub api_key: String,
    #[structopt(long)]
    pub source: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct CreateSlackIntegrationData {
    #[structopt(long, help = "Slack Channel to send messages to")]
    pub channel_id: String,
    #[structopt(long, help = "API token for the Slack bot")]
    pub token: String,
    #[structopt(long)]
    pub source: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct UpdateIntegrationData {
    #[structopt(long, help = "API key used with the Ops Genie integration API")]
    pub api_key: Option<String>,
    #[structopt(long, help = "Slack Channel to send messages to")]
    pub channel_id: Option<String>,
    #[structopt(long, help = "API token for the Slack bot")]
    pub token: Option<String>,
}
