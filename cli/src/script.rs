#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    #[serde(default)]
    debug: bool,

    #[serde(default)]
    json: bool,

    #[serde(rename = "command")]
    commands: Vec<Command>,
}

impl Script {
    // TODO - Discuss if we inject contextual project and org-id value here.
    pub fn commands(self, username: String, password: String) -> impl Iterator<Item = crate::Opt> {
        let debug = self.debug;
        let json = self.json;

        self.commands.into_iter().map(move |cmd| {
            let cmd = match cmd {
                Command::CreateGroup(params) => crate::Command::Access(crate::Access {
                    access_command: crate::AccessCommand::Groups(crate::Groups {
                        groups_command: crate::GroupsCommand::Create(crate::CreateGroup {
                            name: params.name,
                            org_id: params.org_id,
                            members: params.members,
                        }),
                    }),
                }),

                Command::CreateNetwork(params) => crate::Command::Infra(crate::Infra {
                    infra_command: crate::InfraCommand::Networks(crate::Networks {
                        networks_command: crate::NetworksCommand::Create(crate::CreateNetwork {
                            org_id: params.org_id,
                            project_id: params.project_id,
                            provider: params.provider,
                            cidr_block: params.cidr_block,
                            description: params.description,
                            region: params.region,
                        }),
                    }),
                }),
            };

            crate::Opt {
                username: username.clone(),
                password: password.clone(),
                debug,
                json,
                cmd,
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Command {
    CreateGroup(CreateGroup),
    CreateNetwork(CreateNetwork),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CreateGroup {
    pub org_id: esc_api::OrgId,
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CreateNetwork {
    pub org_id: esc_api::OrgId,
    pub project_id: esc_api::ProjectId,
    pub provider: esc_api::Provider,
    pub cidr_block: String,
    pub description: String,
    pub region: String,
}
