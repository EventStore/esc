use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Settings commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    GetSettings(GetSettings),
    Update(UpdateSetting),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::GetSettings(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets the settings of an organization")]
pub struct GetSettings {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl GetSettings {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Settings::new(sender)
            .gets(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates the settings of an organization")]
pub struct UpdateSetting {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub require_mfa: Option<bool>,
    #[structopt(long)]
    pub restrict_invite_domain: Option<String>,
}

impl UpdateSetting {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Settings::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::access::UpdateSettingsRequest {
                    require_mfa: self.require_mfa.clone(),
                    restrict_invite_domain: self.restrict_invite_domain.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
