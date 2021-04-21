use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Organizations commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListOrganizations),
    Create(CreateOrganization),
    Delete(DeleteOrganization),
    Get(GetOrganization),
    Update(UpdateOrganization),
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
#[structopt(about = "Lists organizations under the account owned by the credentials")]
pub struct ListOrganizations {}

impl ListOrganizations {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Organizations::new(sender)
            .list()
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, crate::List(result.organizations))?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new organization")]
pub struct CreateOrganization {
    #[structopt(long)]
    pub name: String,
}

impl CreateOrganization {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Organizations::new(sender)
            .create(esc_api::resources::CreateOrganizationRequest {
                name: self.name.clone(),
            })
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, result.id)?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deletes an organization by ID.")]
pub struct DeleteOrganization {
    #[structopt(long, help="The id of the organization to delete",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl DeleteOrganization {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::resources::paths::Organizations::new(sender)
            .delete(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Gets a single organization by ID.")]
pub struct GetOrganization {
    #[structopt(long, help="The id of the organization to retrieve",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl GetOrganization {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Organizations::new(sender)
            .get(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, result.organization)?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deletes an organization by ID.")]
pub struct UpdateOrganization {
    #[structopt(long, help="The id of the organization to alter",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub name: String,
}

impl UpdateOrganization {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Organizations::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::resources::UpdateOrganizationRequest {
                    name: self.name.clone(),
                },
            )
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, result.organization)?;
        }

        Ok(())
    }
}
