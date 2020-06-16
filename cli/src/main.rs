#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod config;
mod constants;
mod script;
mod store;

use crate::store::{Auth, TokenStore};
use esc_api::{Client, ClientId, GroupId, OrgId};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "esc",
    about = "EventStoreDB Cloud tool.",
    author = "Event Store Limited <ops@eventstore.com>"
)]
pub struct Opt {
    #[structopt(long, short, env = "ESC_USERNAME", default_value = "")]
    username: String,

    #[structopt(
        long,
        short,
        env = "ESC_PASSWORD",
        default_value = "",
        hide_env_values = true
    )]
    password: String,

    #[structopt(long)]
    debug: bool,

    #[structopt(long)]
    json: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Access(Access),
    Resources(Resources),
    Infra(Infra),
    Context(Context),
    Script(Script),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "API access calls")]
struct Access {
    #[structopt(subcommand)]
    access_command: AccessCommand,
}

#[derive(StructOpt, Debug)]
enum AccessCommand {
    Tokens(Tokens),
    Groups(Groups),
}

#[derive(StructOpt, Debug)]
struct Tokens {
    #[structopt(subcommand)]
    token_command: TokensCommand,
}

#[derive(StructOpt, Debug)]
enum TokensCommand {
    User(User),
}

#[derive(StructOpt, Debug)]
struct Groups {
    #[structopt(subcommand)]
    groups_command: GroupsCommand,
}

#[derive(StructOpt, Debug)]
struct User {
    #[structopt(subcommand)]
    user_command: UserCommand,
}

#[derive(StructOpt, Debug)]
enum GroupsCommand {
    Create(CreateGroup),
    Update(UpdateGroup),
    Delete(DeleteGroup),
}

#[derive(StructOpt, Debug)]
enum UserCommand {
    List,
    Revoke,
}

#[derive(StructOpt, Debug)]
struct CreateGroup {
    #[structopt(long, short)]
    name: String,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, short)]
    members: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct UpdateGroup {
    #[structopt(long, short)]
    id: String,

    #[structopt(long, short)]
    name: Option<String>,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, short)]
    members: Option<Vec<String>>,
}

#[derive(StructOpt, Debug)]
struct DeleteGroup {
    #[structopt(long, short)]
    id: String,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "API infra calls")]
struct Infra {
    #[structopt(subcommand)]
    infra_command: InfraCommand,
}

#[derive(StructOpt, Debug)]
enum InfraCommand {
    Networks(Networks),
}

#[derive(StructOpt, Debug)]
struct Networks {
    #[structopt(subcommand)]
    networks_command: NetworksCommand,
}

#[derive(StructOpt, Debug)]
enum NetworksCommand {
    Create(CreateNetwork),
    Delete(DeleteNetwork),
    Get(GetNetwork),
    List(ListNetworks),
    Update(UpdateNetwork),
}

#[derive(StructOpt, Debug)]
struct CreateNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_provider))]
    provider: esc_api::Provider,

    #[structopt(long)]
    cidr_block: String,

    #[structopt(long)]
    description: String,

    #[structopt(long)]
    region: String,
}

#[derive(StructOpt, Debug)]
struct DeleteNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id))]
    id: esc_api::NetworkId,
}

#[derive(StructOpt, Debug)]
struct GetNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id))]
    id: esc_api::NetworkId,
}

#[derive(StructOpt, Debug)]
struct ListNetworks {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,
}

#[derive(StructOpt, Debug)]
struct UpdateNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id))]
    id: esc_api::NetworkId,

    #[structopt(long)]
    description: String,
}

#[derive(StructOpt, Debug)]
struct Context {
    #[structopt(subcommand)]
    context_command: ContextCommand,
}

#[derive(StructOpt, Debug)]
enum ContextCommand {
    Set(ContextProp),
    Get(NamedProp),
    Delete(NamedProp),
    List,
}

#[derive(StructOpt, Debug)]
struct ContextProp {
    #[structopt(long, short, parse(try_from_str = parse_context_prop_name))]
    name: ContextPropName,

    #[structopt(long, short)]
    value: String,
}

#[derive(StructOpt, Debug)]
struct NamedProp {
    #[structopt(long, short, parse(try_from_str = parse_context_prop_name))]
    name: ContextPropName,
}

#[derive(Debug, Copy, Clone)]
enum ContextPropName {
    OrgId,
    ProjectId,
}

#[derive(Debug, StructOpt)]
struct Script {
    #[structopt(long, short, parse(try_from_str = parse_command_script))]
    script: script::Script,
}

#[derive(Debug, StructOpt)]
struct Resources {
    #[structopt(subcommand)]
    resources_command: ResourcesCommand,
}

#[derive(Debug, StructOpt)]
enum ResourcesCommand {
    Organizations(Organizations),
}

#[derive(Debug, StructOpt)]
struct Organizations {
    #[structopt(subcommand)]
    organizations_command: OrganizationsCommand,
}

#[derive(Debug, StructOpt)]
enum OrganizationsCommand {
    List,
}

lazy_static! {
    static ref PROVIDERS: HashMap<&'static str, esc_api::Provider> = {
        let mut map = HashMap::new();
        map.insert("aws", esc_api::Provider::AWS);
        map.insert("gcp", esc_api::Provider::GCP);
        map.insert("azure", esc_api::Provider::AZURE);
        map
    };
}

lazy_static! {
    static ref CONTEXT_PROP_NAMES: HashMap<&'static str, ContextPropName> = {
        let mut map = HashMap::new();
        map.insert("project-id", ContextPropName::ProjectId);
        map.insert("org-id", ContextPropName::OrgId);
        map
    };
}

fn parse_org_id(src: &str) -> Result<esc_api::OrgId, String> {
    if src.trim().is_empty() {
        if let Some(value) = crate::config::SETTINGS
            .context
            .as_ref()
            .and_then(|c| c.org_id.as_ref())
        {
            return Ok(value.clone());
        }

        return Err("Not provided and you don't have an org-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(esc_api::OrgId(src.to_string()))
}

fn parse_project_id(src: &str) -> Result<esc_api::ProjectId, String> {
    if src.trim().is_empty() {
        if let Some(value) = crate::config::SETTINGS
            .context
            .as_ref()
            .and_then(|c| c.project_id.as_ref())
        {
            return Ok(value.clone());
        }

        return Err("Not provided and you don't have an project-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(esc_api::ProjectId(src.to_string()))
}

fn parse_network_id(src: &str) -> Result<esc_api::NetworkId, String> {
    Ok(esc_api::NetworkId(src.to_string()))
}

fn parse_provider(src: &str) -> Result<esc_api::Provider, String> {
    match PROVIDERS.get(src) {
        Some(p) => Ok(*p),
        None => {
            let supported: Vec<&&str> = PROVIDERS.keys().collect();
            Err(format!(
                "Unsupported provider: \"{}\". Supported values: {:?}",
                src, supported
            ))
        }
    }
}

fn parse_context_prop_name(src: &str) -> Result<ContextPropName, String> {
    match CONTEXT_PROP_NAMES.get(src) {
        Some(p) => Ok(*p),
        None => {
            let supported: Vec<&&str> = CONTEXT_PROP_NAMES.keys().collect();
            Err(format!(
                "Unsupported context property name: \"{}\". Supported values: {:?}",
                src, supported
            ))
        }
    }
}

fn parse_command_script(src: &str) -> Result<script::Script, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(src)?;
    let script = toml::from_slice(bytes.as_slice())?;
    Ok(script)
}

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for StringError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let audience = constants::ES_CLOUD_API_AUDIENCE.parse()?;
    let auth = Auth {
        id: ClientId(constants::ES_CLIENT_ID.to_owned()),
        username: opt.username.clone(),
        password: opt.password.clone(),
        audience,
    };
    let client = Client::new(
        constants::ES_CLOUD_API_URL.to_string(),
        constants::ES_CLOUD_IDENTITY_URL.to_string(),
    );
    let mut store = TokenStore::new(auth, client.tokens());
    store.configure().await?;

    if opt.debug {
        std::env::set_var("RUST_LOG", "esc_api=debug,esc=debug");
        env_logger::init();
    }

    let mut work_items: Box<dyn Iterator<Item = Opt>> = Box::new(std::iter::once(opt));

    loop {
        if let Some(opt) = work_items.next() {
            match opt.cmd {
                Command::Access(access) => {
                    match access.access_command {
                        AccessCommand::Groups(groups) => match groups.groups_command {
                            GroupsCommand::Create(params) => {
                                let token = store.access().await?;
                                let create_params = esc_api::command::groups::CreateGroupParams {
                                    org_id: params.org_id,
                                    name: params.name,
                                    members: params.members,
                                };
                                let group_id = client.groups(&token).create(create_params).await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &group_id)?;
                                } else {
                                    println!("{}", group_id);
                                }
                            }

                            GroupsCommand::Update(params) => {
                                let token = store.access().await?;
                                let mut update_group = client
                                    .groups(&token)
                                    .update(GroupId(params.id), params.org_id);

                                update_group.set_name(params.name);
                                update_group.set_members(params.members);
                                update_group.execute().await?;
                            }

                            GroupsCommand::Delete(params) => {
                                let token = store.access().await?;
                                client
                                    .groups(&token)
                                    .delete(GroupId(params.id), params.org_id)
                                    .await?;
                            }
                        },

                        ignored => {
                            println!("$> {:?}", ignored);
                            unimplemented!()
                        }
                    }

                    break;
                }

                Command::Infra(infra) => {
                    match infra.infra_command {
                        InfraCommand::Networks(networks) => match networks.networks_command {
                            NetworksCommand::Create(params) => {
                                let token = store.access().await?;
                                let create_params =
                                    esc_api::command::networks::CreateNetworkParams {
                                        provider: params.provider,
                                        cidr_block: params.cidr_block,
                                        description: params.description,
                                        region: params.region,
                                    };
                                let network_id = client
                                    .networks(&token)
                                    .create(params.org_id, params.project_id, create_params)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &network_id)?;
                                } else {
                                    println!("{}", network_id);
                                }
                            }

                            NetworksCommand::Update(params) => {
                                let token = store.access().await?;
                                let update_params =
                                    esc_api::command::networks::UpdateNetworkParams {
                                        description: params.description,
                                    };
                                client
                                    .networks(&token)
                                    .update(
                                        params.org_id,
                                        params.project_id,
                                        params.id,
                                        update_params,
                                    )
                                    .await?;
                            }

                            NetworksCommand::Delete(params) => {
                                let token = store.access().await?;
                                client
                                    .networks(&token)
                                    .delete(params.org_id, params.project_id, params.id)
                                    .await?;
                            }

                            NetworksCommand::Get(params) => {
                                let token = store.access().await?;
                                let network = client
                                    .networks(&token)
                                    .get(params.org_id, params.project_id, params.id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &network)?;
                                } else {
                                    println!("{:?}", network);
                                }
                            }

                            NetworksCommand::List(params) => {
                                let token = store.access().await?;
                                let networks = client
                                    .networks(&token)
                                    .list(params.org_id, params.project_id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &networks)?;
                                } else {
                                    for network in networks.into_iter() {
                                        println!("{:?}", network);
                                    }
                                }
                            }
                        },
                    }

                    break;
                }

                Command::Context(context) => {
                    match context.context_command {
                        ContextCommand::Set(prop) => {
                            let mut settings = crate::config::SETTINGS.clone();
                            let mut context = settings.context.unwrap_or_default();

                            match prop.name {
                                ContextPropName::OrgId => {
                                    context.org_id = Some(OrgId(prop.value));
                                }

                                ContextPropName::ProjectId => {
                                    context.project_id = Some(esc_api::ProjectId(prop.value));
                                }
                            }

                            settings.context = Some(context);
                            settings.persist().await?;
                        }

                        ContextCommand::Get(prop) => {
                            let settings = crate::config::SETTINGS.clone();
                            let context = settings.context.unwrap_or_default();

                            match prop.name {
                                ContextPropName::OrgId => {
                                    if let Some(value) = context.org_id {
                                        println!("{}", value);
                                    }

                                    return Err(StringError("Not set".to_string()).into());
                                }

                                ContextPropName::ProjectId => {
                                    if let Some(value) = context.project_id {
                                        println!("{}", value);
                                    }

                                    return Err(StringError("Not set".to_string()).into());
                                }
                            }
                        }

                        ContextCommand::Delete(prop) => {
                            let mut settings = crate::config::SETTINGS.clone();
                            let mut context = settings.context.unwrap_or_default();

                            match prop.name {
                                ContextPropName::OrgId => {
                                    context.org_id = None;
                                }

                                ContextPropName::ProjectId => {
                                    context.project_id = None;
                                }
                            }

                            settings.context = Some(context);
                            settings.persist().await?;
                        }

                        ContextCommand::List => {
                            let settings = crate::config::SETTINGS.clone();
                            let context = settings.context.unwrap_or_default();

                            if opt.json {
                                serde_json::to_writer_pretty(std::io::stdout(), &context)?;
                                return Ok(());
                            }

                            if let Some(value) = context.project_id {
                                println!("project-id = {}", value);
                            }

                            if let Some(value) = context.org_id {
                                println!("org-id = {}", value);
                            }
                        }
                    }

                    break;
                }

                Command::Script(params) => {
                    work_items = Box::new(params.script.commands(opt.username, opt.password));
                }

                Command::Resources(res) => {
                    match res.resources_command {
                        ResourcesCommand::Organizations(orgs) => match orgs.organizations_command {
                            OrganizationsCommand::List => {
                                let token = store.access().await?;
                                let orgs = client.organizations(&token).list().await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &orgs)?;
                                } else {
                                    for org in orgs {
                                        println!(
                                            "id = {}; name = {}; created = {}",
                                            org.id, org.name, org.created
                                        );
                                    }
                                }
                            }
                        },
                    }

                    break;
                }
            };
        }
    }

    Ok(())
}
