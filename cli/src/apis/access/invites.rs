use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Invites commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListInvites),
    Create(CreateInvite),
    ResendInvite(ResendInvite),
    Delete(DeleteInvite),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::ResendInvite(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "lists invites")]
pub struct ListInvites {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl ListInvites {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Invites::new(sender)
            .list(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "creates a new invite")]
pub struct CreateInvite {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub user_email: String,
    #[structopt(long)]
    pub groups: Option<Vec<String>>,
}

impl CreateInvite {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Invites::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::access::CreateInviteRequest {
                    user_email: self.user_email.clone(),
                    groups: match &self.groups {
                        Some(v) => Some(v.iter().map(|e| esc_api::GroupId(e.clone())).collect()),
                        None => None,
                    },
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "resends an invite")]
pub struct ResendInvite {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub id: String,
}

impl ResendInvite {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Invites::new(sender)
            .resend_invite(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::access::ResendInviteRequest {
                    id: esc_api::InviteId(self.id.clone()),
                },
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes an invite")]
pub struct DeleteInvite {
    #[structopt(long, help="The id of the organization",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the invite")]
    pub invite_id: String,
}

impl DeleteInvite {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Invites::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::InviteId(self.invite_id.clone()),
            )
            .await?;

        Ok(())
    }
}
