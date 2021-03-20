#![allow(clippy::unnecessary_wraps)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
use std::sync::Arc;

mod apis;
mod config;
mod constants;
mod enrich;
mod store;

use crate::store::{Auth, TokenStore};
use esc_api::{Client, ClientId, GroupId, OrgId};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "esc",
    about = "EventStoreDB Cloud tool.",
    author = "Event Store Limited <ops@eventstore.com>"
)]
pub struct Opt {
    #[structopt(
        long,
        help = "Prints a verbose output during the program execution",
        global = true
    )]
    debug: bool,

    #[structopt(
        long = "json",
        help = "Render read-command output in JSON",
        global = true
    )]
    render_in_json: bool,

    #[structopt(
        long,
        help = "Refresh token, useful if you intend to use esc in a CI/scripting setting for example",
        global = true
    )]
    refresh_token: Option<String>,

    #[structopt(long, help = "Show all request / response traffic", global = true)]
    show_traffic: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Access(crate::apis::access::Command),
    Infra(crate::apis::infra::Command),
    orchestrate(crate::apis::orchestrate::Command),
    Profiles(Profiles),
    Resources(crate::apis::resources::Command),
    Mesdb(crate::apis::mesdb::Command),
    #[structopt(about = "Prints Bash completion script in STDOUT")]
    GenerateBashCompletion,
    #[structopt(about = "Prints Zsh completion script in STDOUT")]
    GenerateZshCompletion,
    #[structopt(about = "Prints Powershell completion script in STDOUT")]
    GeneratePowershellCompletion,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers ESC local profile management commands")]
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
#[structopt(about = "Set a local profile parameter value")]
struct ProfileProp {
    #[structopt(long, short, help = "The profile's name")]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name), help = "Name of the parameter")]
    name: ProfilePropName,

    #[structopt(long, short, help = "Parameter's value")]
    value: String,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete a profile parameter")]
struct NamedProp {
    #[structopt(long, short, help = "Profile's name")]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name), help = "Name of the parameter")]
    name: ProfilePropName,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read a profile parameter(s)")]
struct OptionalNamedProp {
    #[structopt(long, short, help = "Profile's name")]
    profile: String,

    #[structopt(long, short, parse(try_from_str = parse_context_prop_name), help = "Name of the parameter. If not mentioned, list all the profile's parameters")]
    name: Option<ProfilePropName>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers default profile commands")]
struct ProfileDefault {
    #[structopt(subcommand)]
    default_command: ProfileDefaultCommand,
}

#[derive(StructOpt, Debug)]
enum ProfileDefaultCommand {
    Get(GetProfileDefault),
    Set(ProfileDefaultSet),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read the current default local profile name")]
struct GetProfileDefault {}

#[derive(StructOpt, Debug)]
#[structopt(about = "Set default local profile")]
struct ProfileDefaultSet {
    #[structopt(long, short, help = "Profile's name")]
    value: String,
}

#[derive(Debug, Copy, Clone)]
enum ProfilePropName {
    OrgId,
    ProjectId,
    ApiBaseUrl,
}

lazy_static! {
    static ref PROVIDERS: HashMap<&'static str, esc_api::Provider> = {
        let mut map = HashMap::new();
        map.insert("aws", esc_api::Provider::Aws);
        map.insert("gcp", esc_api::Provider::Gcp);
        map.insert("azure", esc_api::Provider::Azure);
        map
    };
}

lazy_static! {
    static ref CONTEXT_PROP_NAMES: HashMap<&'static str, ProfilePropName> = {
        let mut map = HashMap::new();
        map.insert("project-id", ProfilePropName::ProjectId);
        map.insert("org-id", ProfilePropName::OrgId);
        map.insert("api-base-url", ProfilePropName::ApiBaseUrl);
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

lazy_static! {
    static ref CLUSTER_PROJECTION_LEVELS: HashMap<&'static str, esc_api::ProjectionLevel> = {
        let mut map = HashMap::new();
        map.insert("off", esc_api::ProjectionLevel::Off);
        map.insert("system", esc_api::ProjectionLevel::System);
        map.insert("user", esc_api::ProjectionLevel::User);
        map
    };
}

fn parse_default_org_id(src: &str) -> Result<String, String> {
    if src.trim().is_empty() {
        let profile_opt = crate::config::SETTINGS.get_current_profile();

        if let Some(value) = profile_opt.and_then(|p| p.org_id.as_ref()) {
            return Ok(value.clone().0);
        }

        return Err("Not provided and you don't have an org-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(src.to_string())
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

fn parse_default_project_id(src: &str) -> Result<String, String> {
    if src.trim().is_empty() {
        let profile_opt = crate::config::SETTINGS.get_current_profile();

        if let Some(value) = profile_opt.and_then(|p| p.project_id.as_ref()) {
            return Ok(value.clone().0);
        }

        return Err("Not provided and you don't have an project-id property set in the [context] section of your settings.toml file".to_string());
    }

    Ok(src.to_string())
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

fn parse_backup_id(src: &str) -> Result<esc_api::BackupId, String> {
    Ok(esc_api::BackupId(src.to_string()))
}

fn parse_job_id(src: &str) -> Result<esc_api::JobId, String> {
    Ok(esc_api::JobId(src.to_string()))
}

fn parse_policy_id(src: &str) -> Result<esc_api::PolicyId, String> {
    Ok(esc_api::PolicyId(src.to_string()))
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

fn parse_projection_level(src: &str) -> Result<esc_api::ProjectionLevel, String> {
    parse_enum(&CLUSTER_PROJECTION_LEVELS, src)
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

fn parse_email(src: &str) -> Result<esc_api::Email, String> {
    if let Some(email) = esc_api::Email::parse(src) {
        return Ok(email);
    }

    Err("Invalid email".to_string())
}

fn parse_invite_id(src: &str) -> Result<esc_api::InviteId, String> {
    Ok(esc_api::InviteId(src.to_string()))
}

fn parse_cidr(src: &str) -> Result<cidr::Ipv4Cidr, cidr::NetworkParseError> {
    src.parse()
}

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for StringError {}

fn print_output<A: std::fmt::Debug + Serialize>(
    render_in_json: bool,
    value: A,
) -> Result<(), Box<dyn std::error::Error>> {
    if render_in_json {
        serde_json::to_writer_pretty(std::io::stdout(), &value)?;
    } else {
        println!("{:?}", value);
    }

    Ok(())
}

struct List<A>(Vec<A>);

impl<A> std::fmt::Debug for List<A>
where
    A: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for value in self.0.iter() {
            writeln!(f, "{:?}", value)?;
        }

        Ok(())
    }
}

impl<A> serde::ser::Serialize for List<A>
where
    A: serde::ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for elem in self.0.iter() {
            seq.serialize_element(elem)?;
        }

        seq.end()
    }
}

struct CliConfig {
    client: Client,
    // store: TokenStore<'a>,
    // refresh_token: Option<String>,
    token: esc_api::Token,
    render_in_json: bool,
}

impl CliConfig {
    pub fn create_client(&self) -> Client {
        return self.client.clone();
    }

    // pub async fn get_token(&self) -> Result<esc_api::Token, Box<dyn std::error::Error>> {
    pub fn get_token(&self) -> esc_api::Token {
        // self.store.access(self.refresh_token.clone()).await
        self.token.clone()
    }

    pub fn render_in_json(&self) -> bool {
        self.render_in_json
    }
}

struct TrafficSpy {}

impl esc_api::ClientObserver for TrafficSpy {
    fn on_request(&self, method: &str, url: &str, body: &str) {
        println!("{} {}", method, url);
        if body.len() > 0 {
            println!("{}", body);
        }
    }

    fn on_response(&self, status: &str, body: &str) {
        println!("status: {}", status);
        if body.len() > 0 {
            println!("{}", body);
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut clap_app = Opt::clap();
    let opt = Opt::from_clap(&clap_app.clone().get_matches());
    let audience = constants::ES_CLOUD_API_AUDIENCE.parse()?;
    let auth = Auth {
        id: ClientId(constants::ES_CLIENT_ID.to_owned()),
        audience,
    };
    let base_url = config::SETTINGS
        .get_current_profile()
        .and_then(|profile| {
            profile.api_base_url.as_ref().map(|url| {
                format!(
                    "{}://{}",
                    url.scheme(),
                    url.host_str().expect("Pre-validated it has a host")
                )
            })
        })
        .unwrap_or_else(|| constants::ES_CLOUD_API_URL.to_string());

    let observer: Option<Arc<dyn esc_api::ClientObserver>> = if opt.show_traffic {
        Some(Arc::new(TrafficSpy {}))
    } else {
        None
    };
    let client = Client::builder()
        .set_observer(observer)
        .build(base_url, constants::ES_CLOUD_IDENTITY_URL.to_string())?;

    config::Settings::configure().await?;
    let mut store = TokenStore::new(&auth, client.tokens());
    store.configure().await?;

    if opt.debug {
        std::env::set_var("RUST_LOG", "esc_api=debug,esc=debug");
        env_logger::init();
    }

    match opt.cmd {
        Command::Access(cmd) => {
            let cc = CliConfig {
                client: client.clone(),
                token: store.access(opt.refresh_token).await?,
                render_in_json: opt.render_in_json,
            };
            cmd.command.exec(&cc).await?
        }

        Command::Infra(cmd) => {
            let cc = CliConfig {
                client: client.clone(),
                token: store.access(opt.refresh_token).await?,
                render_in_json: opt.render_in_json,
            };
            cmd.command.exec(&cc).await?
        }

        Command::Mesdb(cmd) => {
            let cc = CliConfig {
                client: client.clone(),
                token: store.access(opt.refresh_token).await?,
                render_in_json: opt.render_in_json,
            };
            cmd.command.exec(&cc).await?
        }

        Command::orchestrate(cmd) => {
            let cc = CliConfig {
                client: client.clone(),
                token: store.access(opt.refresh_token).await?,
                render_in_json: opt.render_in_json,
            };
            cmd.command.exec(&cc).await?
        }

        Command::Resources(cmd) => {
            let cc = CliConfig {
                client: client.clone(),
                token: store.access(opt.refresh_token).await?,
                render_in_json: opt.render_in_json,
            };
            cmd.command.exec(&cc).await?
        }

        Command::Profiles(context) => match context.profiles_command {
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

                    ProfilePropName::ApiBaseUrl => {
                        let url = config::parse_url(params.value.as_str())?;
                        profile.api_base_url = Some(url);
                    }
                }

                settings.persist().await?;
            }

            ProfilesCommand::Get(params) => {
                if let Some(profile) = crate::config::SETTINGS.get_profile(&params.profile) {
                    if let Some(name) = params.name {
                        match name {
                            ProfilePropName::ProjectId => {
                                let default = Default::default();
                                let value = profile.project_id.as_ref().unwrap_or(&default);
                                serde_json::to_writer_pretty(std::io::stdout(), value)?;
                            }

                            ProfilePropName::OrgId => {
                                let default = Default::default();
                                let value = profile.org_id.as_ref().unwrap_or(&default);
                                serde_json::to_writer_pretty(std::io::stdout(), value)?;
                            }

                            ProfilePropName::ApiBaseUrl => {
                                if let Some(url) = profile.api_base_url.as_ref() {
                                    serde_json::to_writer_pretty(std::io::stdout(), url.as_str())?;
                                }
                            }
                        }
                    } else {
                        serde_json::to_writer_pretty(std::io::stdout(), profile)?;
                    }
                }
            }

            ProfilesCommand::List => {
                serde_json::to_writer_pretty(std::io::stdout(), &crate::config::SETTINGS.profiles)?;
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

                    ProfilePropName::ApiBaseUrl => {
                        profile.api_base_url = None;
                    }
                }

                settings.persist().await?;
            }

            ProfilesCommand::Default(default) => match default.default_command {
                ProfileDefaultCommand::Get(_) => {
                    match crate::config::SETTINGS.default_profile.as_ref() {
                        Some(value) => serde_json::to_writer_pretty(std::io::stdout(), value)?,
                        _ => {
                            println!(
                                "No default profile set\n\n\
                            To set a default profile use:\n\n\
                            esc profiles default set <profile_name>"
                            );
                            std::process::exit(-1)
                        }
                    }
                }

                ProfileDefaultCommand::Set(params) => {
                    let mut settings = crate::config::SETTINGS.clone();
                    settings.default_profile = Some(params.value);
                    settings.persist().await?;
                }
            },
        },

        Command::GenerateBashCompletion => {
            clap_app.gen_completions_to("esc", clap::Shell::Bash, &mut std::io::stdout());
        }

        Command::GenerateZshCompletion => {
            clap_app.gen_completions_to("esc", clap::Shell::Zsh, &mut std::io::stdout());
        }

        Command::GeneratePowershellCompletion => {
            clap_app.gen_completions_to("esc", clap::Shell::PowerShell, &mut std::io::stdout());
        }
    };

    Ok(())
}
