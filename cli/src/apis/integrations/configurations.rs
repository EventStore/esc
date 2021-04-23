use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Configurations commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListConfigurations),
    Create(CreateConfiguration),
    Delete(DeleteConfiguration),
    Get(GetConfiguration),
    Update(UpdateConfiguration),
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
#[structopt(about = "list all configurations")]
pub struct ListConfigurations {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListConfigurations {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrations::paths::Configurations::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new configuration")]
pub struct CreateConfiguration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(subcommand)]
    pub data: ConfigurationData,
    #[structopt(long)]
    pub description: String,
}

impl CreateConfiguration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrations::paths::Configurations::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::integrations::CreateConfigurationRequest {
                    data: match &self.data {
                        ConfigurationData::OpsGenie(args) => {
                            esc_api::integrations::ConfigurationData::OpsGenieConfiguration {
                                api_key: args.api_key.clone(),
                            }
                        }
                        ConfigurationData::Slack(args) => {
                            esc_api::integrations::ConfigurationData::SlackConfiguration {
                                channel_id: args.channel_id.clone(),
                                token: args.token.clone(),
                            }
                        }
                    },
                    description: self.description.clone(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a configuration")]
pub struct DeleteConfiguration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the configuration")]
    pub configuration_id: String,
}

impl DeleteConfiguration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::integrations::paths::Configurations::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ConfigurationId(self.configuration_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "retrieves a configuration")]
pub struct GetConfiguration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the configuration")]
    pub configuration_id: String,
}

impl GetConfiguration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::integrations::paths::Configurations::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ConfigurationId(self.configuration_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates a configuration")]
pub struct UpdateConfiguration {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the configuration")]
    pub configuration_id: String,
    #[structopt(subcommand)]
    pub data: ConfigurationData,
    #[structopt(long)]
    pub description: String,
}

impl UpdateConfiguration {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::integrations::paths::Configurations::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ConfigurationId(self.configuration_id.clone()),
                esc_api::integrations::UpdateConfigurationRequest {
                    data: match &self.data {
                        ConfigurationData::OpsGenie(args) => {
                            esc_api::integrations::ConfigurationData::OpsGenieConfiguration {
                                api_key: args.api_key.clone(),
                            }
                        }
                        ConfigurationData::Slack(args) => {
                            esc_api::integrations::ConfigurationData::SlackConfiguration {
                                channel_id: args.channel_id.clone(),
                                token: args.token.clone(),
                            }
                        }
                    },
                    description: self.description.clone(),
                },
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum ConfigurationData {
    OpsGenie(OpsGenieConfiguration),
    Slack(SlackConfiguration),
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct OpsGenieConfiguration {
    #[structopt(long, help = "API key used with the Ops Genie integration API")]
    pub api_key: String,
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct SlackConfiguration {
    #[structopt(long, help = "Slack Channel to send messages to")]
    pub channel_id: String,
    #[structopt(long, help = "API token for the Slack bot")]
    pub token: String,
}
