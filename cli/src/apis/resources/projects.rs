use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Projects commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListProjects),
    Create(CreateProject),
    Delete(DeleteProject),
    Get(GetProject),
    Update(UpdateProject),
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
#[structopt(about = "List projects")]
pub struct ListProjects {
    #[structopt(long, help="The id of the organization to retrieve",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl ListProjects {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Projects::new(sender)
            .list(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, crate::List(result.projects))?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Creates a new project")]
pub struct CreateProject {
    #[structopt(long, help="The id of the organization to retrieve",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub name: String,
}

impl CreateProject {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Projects::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::resources::CreateProjectRequest {
                    name: self.name.clone(),
                },
            )
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
#[structopt(about = "Deletes project")]
pub struct DeleteProject {
    #[structopt(long, help="The id of the organization to delete",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project to delete",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl DeleteProject {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::resources::paths::Projects::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Get project")]
pub struct GetProject {
    #[structopt(long, help="The id of the organization to retrieve",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project to retrieve",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl GetProject {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::resources::paths::Projects::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, result.project)?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Updates a project")]
pub struct UpdateProject {
    #[structopt(long, help="The id of the organization to delete",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project to delete",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long)]
    pub name: String,
}

impl UpdateProject {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::resources::paths::Projects::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::resources::UpdateProjectRequest {
                    name: self.name.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
