use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Policies commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListPolicies),
    Create(CreatePolicy),
    Delete(DeletePolicy),
    Get(GetPolicy),
    Update(UpdatePolicy),
    PolicyAllowed(PolicyAllowed),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
            CommandChoices::PolicyAllowed(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a list of policies")]
pub struct ListPolicies {
    #[structopt(long, help="The organization id the policy will relate to",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
}

impl ListPolicies {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Policies::new(sender)
            .list(esc_api::OrgId(self.organization_id.clone()))
            .await?;

        if render_in_json {
            crate::print_output(render_in_json, result)?;
        } else {
            crate::print_output(render_in_json, crate::List(result.policies))?;
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "creates a new policy")]
pub struct CreatePolicy {
    #[structopt(long, help="The organization id the policy will relate to",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long)]
    pub actions: Vec<String>,
    #[structopt(long, help = "the policy's effect")]
    pub effect: String,
    #[structopt(long, help = "the policy's name")]
    pub name: String,
    #[structopt(long, help = "the policy's resources")]
    pub resources: Vec<String>,
    #[structopt(long, help = "the policy's name")]
    pub subjects: Vec<String>,
}

impl CreatePolicy {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Policies::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::access::CreatePolicyRequest {
                    policy: esc_api::access::CreatePolicy {
                        actions: self
                            .actions
                            .iter()
                            .flat_map(|e| esc_api::access::Action::from_str(&e))
                            .collect(),
                        effect: esc_api::access::Effect::from_str(&self.effect)?,
                        name: self.name.clone(),
                        resources: self.resources.iter().map(|e| e.clone()).collect(),
                        subjects: self.subjects.iter().map(|e| e.clone()).collect(),
                    },
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "deletes a policy")]
pub struct DeletePolicy {
    #[structopt(long, help="The organization id the policy will relate to",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the ID of the policy to delete")]
    pub policy_id: String,
}

impl DeletePolicy {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Policies::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::PolicyId(self.policy_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "gets a single policy")]
pub struct GetPolicy {
    #[structopt(long, help="The organization id the policy will relate to",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the policy")]
    pub policy_id: String,
}

impl GetPolicy {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Policies::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::PolicyId(self.policy_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "modifies the policy")]
pub struct UpdatePolicy {
    #[structopt(long, help="The organization id the policy will relate to",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help = "the id of the policy")]
    pub policy_id: String,
    #[structopt(long)]
    pub actions: Vec<String>,
    #[structopt(long, help = "the policy's effect")]
    pub effect: String,
    #[structopt(long)]
    pub name: String,
    #[structopt(long)]
    pub resources: Vec<String>,
    #[structopt(long)]
    pub subjects: Vec<String>,
}

impl UpdatePolicy {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::access::paths::Policies::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::PolicyId(self.policy_id.clone()),
                esc_api::access::UpdatePolicyRequest {
                    policy: esc_api::access::UpdatePolicy {
                        actions: self
                            .actions
                            .iter()
                            .flat_map(|e| esc_api::access::Action::from_str(&e))
                            .collect(),
                        effect: esc_api::access::Effect::from_str(&self.effect)?,
                        name: self.name.clone(),
                        resources: self.resources.iter().map(|e| e.clone()).collect(),
                        subjects: self.subjects.iter().map(|e| e.clone()).collect(),
                    },
                },
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "checks to see if a policy is accepted")]
pub struct PolicyAllowed {
    #[structopt(long)]
    pub action: String,
    #[structopt(long, help = "describes what a subject of athe policy's actions")]
    pub resource: String,
}

impl PolicyAllowed {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::access::paths::Policies::new(sender)
            .policy_allowed(esc_api::access::PolicyAllowedRequest {
                action: self.action.clone(),
                resource: esc_api::access::Action::from_str(&self.resource)?,
            })
            .await?;

        crate::print_output(render_in_json, result)
    }
}
