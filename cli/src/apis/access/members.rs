use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Members commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListMembers),
    Delete(DeleteMember),
    Get(GetMember),
    Update(UpdateMember),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "lists members")]
pub struct ListMembers {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl ListMembers {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Members::new(sender)
            .list(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a member")]
pub struct DeleteMember {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the member")]
    pub member_id: String,
}

impl DeleteMember {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Members::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                &self.member_id,
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a member")]
pub struct GetMember {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the member")]
    pub member_id: String,
}

impl GetMember {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Members::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                &self.member_id,
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "updates the member")]
pub struct UpdateMember {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "TODO")]
    pub member_id: String,
    #[structopt(long)]
    pub active: bool,
}

impl UpdateMember {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Members::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                &self.member_id,
                esc_api::access::UpdateMemberRequest {
                    active: self.active.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
