use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Groups commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListGroups),
    Create(CreateGroup),
    Delete(DeleteGroup),
    Get(GetGroup),
    Update(UpdateGroup),
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
#[structopt(about = "list groups")]
pub struct ListGroups {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "Filter by linked resource")]
    pub linked_resource: Option<String>,
}

impl ListGroups {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Groups::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                self.linked_resource.clone(),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "creates a new group")]
pub struct CreateGroup {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub name: String,
    #[structopt(long)]
    pub members: Option<Vec<String>>,
}

impl CreateGroup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Groups::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::access::CreateGroupRequest {
                    name: self.name.clone(),
                    members: match &self.members {
                        Some(v) => Some(v.iter().map(|e| e.clone()).collect()),
                        None => None,
                    },
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a group")]
pub struct DeleteGroup {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the group")]
    pub group_id: String,
}

impl DeleteGroup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Groups::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::GroupId(self.group_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a single group")]
pub struct GetGroup {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the group")]
    pub group_id: String,
}

impl GetGroup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Groups::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::GroupId(self.group_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates a group")]
pub struct UpdateGroup {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the group")]
    pub group_id: String,
    #[structopt(long)]
    pub name: Option<String>,
    #[structopt(long)]
    pub members: Option<Vec<String>>,
}

impl UpdateGroup {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Groups::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::GroupId(self.group_id.clone()),
                esc_api::access::UpdateGroupRequest {
                    name: self.name.clone(),
                    members: match &self.members {
                        Some(v) => Some(v.iter().map(|e| e.clone()).collect()),
                        None => None,
                    },
                },
            )
            .await?;

        Ok(())
    }
}
