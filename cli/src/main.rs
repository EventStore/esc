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
    Profiles(Profiles),
    Script(Script),
    Mesdb(Mesdb),
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
    Get(GetGroup),
    Delete(DeleteGroup),
    List(ListGroups),
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
    #[structopt(long, short, parse(try_from_str = parse_group_id))]
    id: GroupId,

    #[structopt(long, short)]
    name: Option<String>,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, short)]
    members: Option<Vec<String>>,
}

#[derive(StructOpt, Debug)]
struct GetGroup {
    #[structopt(long, short, parse(try_from_str = parse_group_id))]
    id: GroupId,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
struct DeleteGroup {
    #[structopt(long, short, parse(try_from_str = parse_group_id))]
    id: GroupId,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
struct ListGroups {
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
    Peerings(Peerings),
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
struct Peerings {
    #[structopt(subcommand)]
    peerings_command: PeeringsCommand,
}

#[derive(StructOpt, Debug)]
enum PeeringsCommand {
    Create(CreatePeering),
    Delete(DeletePeering),
    Get(GetPeering),
    List(ListPeerings),
    Update(UpdatePeering),
}

#[derive(StructOpt, Debug)]
struct CreatePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_network_id), default_value = "")]
    network_id: esc_api::NetworkId,

    #[structopt(long)]
    peer_account_id: String,

    #[structopt(long)]
    peer_network_id: String,

    #[structopt(long)]
    description: String,

    #[structopt(long)]
    peer_network_region: String,

    #[structopt(long)]
    routes: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct DeletePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id))]
    id: esc_api::PeeringId,
}

#[derive(StructOpt, Debug)]
struct GetPeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id))]
    id: esc_api::PeeringId,
}

#[derive(StructOpt, Debug)]
struct ListPeerings {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,
}

#[derive(StructOpt, Debug)]
struct UpdatePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id))]
    id: esc_api::PeeringId,

    #[structopt(long)]
    description: String,
}

#[derive(StructOpt, Debug)]
struct Profiles {
    #[structopt(subcommand)]
    profiles_command: ProfilesCommand,
}

#[derive(StructOpt, Debug)]
enum ProfilesCommand {
    Set(ProfileProp),
    Get(OptionalNamedProp),
    Delete(NamedProp),
    List,
    Default(ProfileDefault),
}

#[derive(StructOpt, Debug)]
struct ProfileProp {
    #[structopt(long, short)]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name))]
    name: ProfilePropName,

    #[structopt(long, short)]
    value: String,
}

#[derive(StructOpt, Debug)]
struct NamedProp {
    #[structopt(long, short)]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name))]
    name: ProfilePropName,
}

#[derive(StructOpt, Debug)]
struct OptionalNamedProp {
    #[structopt(long, short)]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name))]
    name: Option<ProfilePropName>,
}

#[derive(StructOpt, Debug)]
struct ProfileDefault {
    #[structopt(subcommand)]
    default_command: ProfileDefaultCommand,
}

#[derive(StructOpt, Debug)]
enum ProfileDefaultCommand {
    Get,
    Set(ProfileDefaultSet),
}

#[derive(StructOpt, Debug)]
struct ProfileDefaultSet {
    #[structopt(long, short)]
    value: String,
}

#[derive(Debug, Copy, Clone)]
enum ProfilePropName {
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
    Projects(Projects),
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

#[derive(Debug, StructOpt)]
struct Projects {
    #[structopt(subcommand)]
    projects_command: ProjectsCommand,
}

#[derive(Debug, StructOpt)]
enum ProjectsCommand {
    Create(CreateProject),
    Update(UpdateProject),
    Get(GetProject),
    Delete(DeleteProject),
    List(ListProjects),
}

#[derive(Debug, StructOpt)]
struct CreateProject {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, short)]
    name: String,
}

#[derive(Debug, StructOpt)]
struct UpdateProject {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short)]
    name: String,
}

#[derive(Debug, StructOpt)]
struct GetProject {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_project_id), default_value = "")]
    id: esc_api::ProjectId,
}

#[derive(Debug, StructOpt)]
struct DeleteProject {
    #[structopt(long, parse(try_from_str = parse_org_id))]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_project_id))]
    id: esc_api::ProjectId,
}

#[derive(Debug, StructOpt)]
struct ListProjects {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,
}

#[derive(Debug, StructOpt)]
struct Mesdb {
    #[structopt(subcommand)]
    mesdb_command: MesdbCommand,
}

#[derive(Debug, StructOpt)]
enum MesdbCommand {
    Clusters(Clusters),
}

#[derive(Debug, StructOpt)]
struct Clusters {
    #[structopt(subcommand)]
    clusters_command: ClustersCommand,
}

#[derive(Debug, StructOpt)]
enum ClustersCommand {
    Create(CreateCluster),
    Get(GetCluster),
    List(ListClusters),
    Update(UpdateCluster),
    Delete(DeleteCluster),
}

#[derive(Debug, StructOpt)]
struct CreateCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_network_id))]
    network_id: esc_api::NetworkId,

    #[structopt(long)]
    description: String,

    #[structopt(long, parse(try_from_str = parse_topology))]
    topology: esc_api::Topology,

    #[structopt(long)]
    instance_type: String,

    #[structopt(long)]
    disk_size_in_gb: usize,

    #[structopt(long)]
    disk_type: String,

    #[structopt(long)]
    server_version: String,
}

#[derive(Debug, StructOpt)]
struct GetCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id))]
    id: esc_api::ClusterId,
}

#[derive(Debug, StructOpt)]
struct ListClusters {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,
}

#[derive(Debug, StructOpt)]
struct UpdateCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id))]
    id: esc_api::ClusterId,
}

#[derive(Debug, StructOpt)]
struct DeleteCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id))]
    id: esc_api::ClusterId,
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
    static ref CONTEXT_PROP_NAMES: HashMap<&'static str, ProfilePropName> = {
        let mut map = HashMap::new();
        map.insert("project-id", ProfilePropName::ProjectId);
        map.insert("org-id", ProfilePropName::OrgId);
        map
    };
}

lazy_static! {
    static ref CLUSTER_TOPOLOGIES: HashMap<&'static str, esc_api::Topology> = {
        let mut map = HashMap::new();
        map.insert("single-node", esc_api::Topology::SingleNode);
        map.insert(
            "three-node-multi-zone",
            esc_api::Topology::ThreeNodeMultiZone,
        );
        map
    };
}

fn parse_org_id(src: &str) -> Result<esc_api::OrgId, String> {
    if src.trim().is_empty() {
        let profile_opt = crate::config::SETTINGS.get_current_profile();

        if let Some(value) = profile_opt.and_then(|p| p.org_id.as_ref()) {
            return Ok(value.clone());
        }

        return Err("Not provided and you don't have an org-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(esc_api::OrgId(src.to_string()))
}

fn parse_project_id(src: &str) -> Result<esc_api::ProjectId, String> {
    if src.trim().is_empty() {
        let profile_opt = crate::config::SETTINGS.get_current_profile();

        if let Some(value) = profile_opt.and_then(|p| p.project_id.as_ref()) {
            return Ok(value.clone());
        }

        return Err("Not provided and you don't have an project-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(esc_api::ProjectId(src.to_string()))
}

fn parse_network_id(src: &str) -> Result<esc_api::NetworkId, String> {
    Ok(esc_api::NetworkId(src.to_string()))
}

fn parse_group_id(src: &str) -> Result<esc_api::GroupId, String> {
    Ok(esc_api::GroupId(src.to_string()))
}

fn parse_peering_id(src: &str) -> Result<esc_api::PeeringId, String> {
    Ok(esc_api::PeeringId(src.to_string()))
}

fn parse_cluster_id(src: &str) -> Result<esc_api::ClusterId, String> {
    Ok(esc_api::ClusterId(src.to_string()))
}

fn parse_provider(src: &str) -> Result<esc_api::Provider, String> {
    parse_enum(&PROVIDERS, src)
}

fn parse_context_prop_name(src: &str) -> Result<ProfilePropName, String> {
    parse_enum(&CONTEXT_PROP_NAMES, src)
}

fn parse_topology(src: &str) -> Result<esc_api::Topology, String> {
    parse_enum(&CLUSTER_TOPOLOGIES, src)
}

fn parse_enum<A: Copy>(env: &'static HashMap<&'static str, A>, src: &str) -> Result<A, String> {
    match env.get(src) {
        Some(p) => Ok(*p),
        None => {
            let supported: Vec<&&str> = env.keys().collect();
            Err(format!(
                "Unsupported value: \"{}\". Supported values: {:?}",
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
                                let mut update_group =
                                    client.groups(&token).update(params.id, params.org_id);

                                update_group.set_name(params.name);
                                update_group.set_members(params.members);
                                update_group.execute().await?;
                            }

                            GroupsCommand::Get(params) => {
                                let token = store.access().await?;
                                let group_id_opt =
                                    client.groups(&token).get(params.id, params.org_id).await?;

                                match group_id_opt {
                                    Some(group) => {
                                        if opt.json {
                                            serde_json::to_writer_pretty(
                                                std::io::stdout(),
                                                &group,
                                            )?;
                                        } else {
                                            println!("id = {}; org-id = {}; name = {}, created = {}, members = {:?}", group.id, group.org_id, group.name, group.name, group.members);
                                        }
                                    }

                                    _ => {
                                        eprintln!("Group doesn't exists");
                                        std::process::exit(-1);
                                    }
                                }
                            }

                            GroupsCommand::Delete(params) => {
                                let token = store.access().await?;
                                client
                                    .groups(&token)
                                    .delete(params.id, params.org_id)
                                    .await?;
                            }

                            GroupsCommand::List(params) => {
                                let token = store.access().await?;
                                let groups = client.groups(&token).list(params.org_id).await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &groups)?;
                                } else {
                                    for group in groups {
                                        println!("id = {}; org-id = {}; name = {}, created = {}, members = {:?}", group.id, group.org_id, group.name, group.name, group.members);
                                    }
                                }
                            }
                        },

                        ignored => {
                            println!("$> {:?}", ignored);
                            unimplemented!()
                        }
                    }

                    continue;
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

                        InfraCommand::Peerings(peerings) => match peerings.peerings_command {
                            PeeringsCommand::Create(params) => {
                                let token = store.access().await?;
                                let create_params =
                                    esc_api::command::peerings::CreatePeeringParams {
                                        network_id: params.network_id,
                                        description: params.description,
                                        peer_account: params.peer_account_id,
                                        peer_network: params.peer_network_id,
                                        peer_network_region: params.peer_network_region,
                                        routes: params.routes,
                                    };
                                let peering_id = client
                                    .peerings(&token)
                                    .create(params.org_id, params.project_id, create_params)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &peering_id)?;
                                } else {
                                    println!("{}", peering_id);
                                }
                            }

                            PeeringsCommand::Update(params) => {
                                let token = store.access().await?;
                                let update_params =
                                    esc_api::command::peerings::UpdatePeeringParams {
                                        description: params.description,
                                    };
                                client
                                    .peerings(&token)
                                    .update(
                                        params.org_id,
                                        params.project_id,
                                        params.id,
                                        update_params,
                                    )
                                    .await?;
                            }

                            PeeringsCommand::Delete(params) => {
                                let token = store.access().await?;
                                client
                                    .peerings(&token)
                                    .delete(params.org_id, params.project_id, params.id)
                                    .await?;
                            }

                            PeeringsCommand::Get(params) => {
                                let token = store.access().await?;
                                let peering = client
                                    .peerings(&token)
                                    .get(params.org_id, params.project_id, params.id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &peering)?;
                                } else {
                                    println!("{:?}", peering);
                                }
                            }

                            PeeringsCommand::List(params) => {
                                let token = store.access().await?;
                                let peerings = client
                                    .peerings(&token)
                                    .list(params.org_id, params.project_id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &peerings)?;
                                } else {
                                    for peering in peerings.into_iter() {
                                        println!("{:?}", peering);
                                    }
                                }
                            }
                        },
                    }

                    continue;
                }

                Command::Profiles(context) => {
                    match context.profiles_command {
                        ProfilesCommand::Set(params) => {
                            let mut settings = crate::config::SETTINGS.clone();
                            let profile = settings.get_profile_mut(&params.profile);

                            match params.name {
                                ProfilePropName::ProjectId => {
                                    profile.project_id = Some(esc_api::ProjectId(params.value));
                                }

                                ProfilePropName::OrgId => {
                                    profile.org_id = Some(esc_api::OrgId(params.value));
                                }
                            }

                            settings.persist().await?;
                        }

                        ProfilesCommand::Get(params) => {
                            if let Some(profile) =
                                crate::config::SETTINGS.get_profile(&params.profile)
                            {
                                if let Some(name) = params.name {
                                    match name {
                                        ProfilePropName::ProjectId => {
                                            let default = Default::default();
                                            let value =
                                                profile.project_id.as_ref().unwrap_or(&default);
                                            serde_json::to_writer_pretty(std::io::stdout(), value)?;
                                        }

                                        ProfilePropName::OrgId => {
                                            let default = Default::default();
                                            let value = profile.org_id.as_ref().unwrap_or(&default);
                                            serde_json::to_writer_pretty(std::io::stdout(), value)?;
                                        }
                                    }
                                } else {
                                    serde_json::to_writer_pretty(std::io::stdout(), profile)?;
                                }
                            }
                        }

                        ProfilesCommand::List => {
                            serde_json::to_writer_pretty(
                                std::io::stdout(),
                                &crate::config::SETTINGS.profiles,
                            )?;
                        }

                        ProfilesCommand::Delete(params) => {
                            let mut settings = crate::config::SETTINGS.clone();
                            let profile = settings.get_profile_mut(&params.profile);

                            match params.name {
                                ProfilePropName::ProjectId => {
                                    profile.project_id = None;
                                }

                                ProfilePropName::OrgId => {
                                    profile.org_id = None;
                                }
                            }

                            settings.persist().await?;
                        }

                        ProfilesCommand::Default(default) => match default.default_command {
                            ProfileDefaultCommand::Get => {
                                match crate::config::SETTINGS.default_profile.as_ref() {
                                    Some(value) => {
                                        serde_json::to_writer_pretty(std::io::stdout(), value)?
                                    }
                                    _ => std::process::exit(-1),
                                }
                            }

                            ProfileDefaultCommand::Set(params) => {
                                let mut settings = crate::config::SETTINGS.clone();
                                settings.default_profile = Some(params.value);
                                settings.persist().await?;
                            }
                        },
                    }

                    continue;
                }

                Command::Script(params) => {
                    work_items = Box::new(params.script.commands(opt.username, opt.password));
                    continue;
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

                        ResourcesCommand::Projects(projs) => match projs.projects_command {
                            ProjectsCommand::Create(params) => {
                                let token = store.access().await?;
                                let proj_id = client
                                    .projects(&token)
                                    .create(params.org_id, params.name)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &proj_id)?;
                                } else {
                                    println!("{}", proj_id);
                                }
                            }

                            ProjectsCommand::Update(params) => {
                                let token = store.access().await?;
                                client
                                    .projects(&token)
                                    .update(params.org_id, params.project_id, params.name)
                                    .await?;
                            }

                            ProjectsCommand::Get(params) => {
                                let token = store.access().await?;
                                let project_opt = client
                                    .projects(&token)
                                    .get(params.org_id, params.id)
                                    .await?;

                                match project_opt {
                                    Some(proj) => {
                                        if opt.json {
                                            serde_json::to_writer_pretty(std::io::stdout(), &proj)?;
                                        } else {
                                            println!(
                                                "id = {}; name = {}; org-id = {}; created = {}",
                                                proj.id, proj.name, proj.org_id, proj.created
                                            );
                                        }
                                    }

                                    _ => {
                                        eprintln!("Project doesn't exists");
                                        std::process::exit(-1);
                                    }
                                }
                            }

                            ProjectsCommand::Delete(params) => {
                                let token = store.access().await?;
                                client
                                    .projects(&token)
                                    .delete(params.org_id, params.id)
                                    .await?;
                            }

                            ProjectsCommand::List(params) => {
                                let token = store.access().await?;
                                let projs = client.projects(&token).list(params.org_id).await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &projs)?;
                                } else {
                                    for proj in projs {
                                        println!(
                                            "id = {}; name = {}; org-id = {}; created = {}",
                                            proj.id, proj.name, proj.org_id, proj.created
                                        );
                                    }
                                }
                            }
                        },
                    }

                    continue;
                }

                Command::Mesdb(mesdb) => {
                    let token = store.access().await?;
                    match mesdb.mesdb_command {
                        MesdbCommand::Clusters(clusters) => match clusters.clusters_command {
                            ClustersCommand::Create(params) => {
                                let create_params =
                                    esc_api::command::clusters::CreateClusterParams {
                                        network_id: params.network_id,
                                        description: params.description,
                                        topology: params.topology,
                                        instance_type: params.instance_type,
                                        disk_size_gb: params.disk_size_in_gb,
                                        disk_type: params.disk_type,
                                        server_version: params.server_version,
                                    };
                                let cluster_id = client
                                    .clusters(&token)
                                    .create(params.org_id, params.project_id, create_params)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &cluster_id)?;
                                } else {
                                    println!("{}", cluster_id);
                                }
                            }

                            ClustersCommand::Get(params) => {
                                let cluster = client
                                    .clusters(&token)
                                    .get(params.org_id, params.project_id, params.id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &cluster)?;
                                } else {
                                    println!("{:?}", cluster);
                                }
                            }

                            ClustersCommand::Delete(params) => {
                                client
                                    .clusters(&token)
                                    .delete(params.org_id, params.project_id, params.id)
                                    .await?;
                            }

                            ClustersCommand::Update(params) => {
                                client
                                    .clusters(&token)
                                    .update(params.org_id, params.project_id, params.id)
                                    .await?;
                            }

                            ClustersCommand::List(params) => {
                                let clusters = client
                                    .clusters(&token)
                                    .list(params.org_id, params.project_id)
                                    .await?;

                                if opt.json {
                                    serde_json::to_writer_pretty(std::io::stdout(), &clusters)?;
                                } else {
                                    for cluster in clusters {
                                        println!("{:?}", cluster);
                                    }
                                }
                            }
                        },
                    }

                    continue;
                }
            };
        }

        break;
    }

    Ok(())
}
