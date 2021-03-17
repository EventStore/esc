#![allow(clippy::unnecessary_wraps)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

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

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Access(Access),
    Resources(crate::apis::resources::Resources),
    Infra(crate::apis::infra::Infra),
    Profiles(Profiles),
    Mesdb(Mesdb),
    #[structopt(about = "Prints Bash completion script in STDOUT")]
    GenerateBashCompletion,
    #[structopt(about = "Prints Zsh completion script in STDOUT")]
    GenerateZshCompletion,
    #[structopt(about = "Prints Powershell completion script in STDOUT")]
    GeneratePowershellCompletion,
}

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Gathers tokens, groups, members, invites, policies and settings management commands"
)]
struct Access {
    #[structopt(subcommand)]
    access_command: AccessCommand,
}

#[derive(StructOpt, Debug)]
enum AccessCommand {
    Tokens(Tokens),
    Groups(Groups),
    Invites(Invites),
    Policies(Policies),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers tokens management commands")]
struct Tokens {
    #[structopt(subcommand)]
    tokens_command: TokensCommand,
}

#[derive(StructOpt, Debug)]
enum TokensCommand {
    Create(CreateToken),
    Display(Display),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Create an access token")]
struct CreateToken {
    #[structopt(long, short, parse(try_from_str = parse_email), help = "The email you used to create an EventStoreDB Cloud")]
    email: esc_api::Email,

    #[structopt(
        long,
        help = "Set this parameter if you don't want to give your password safely (non-interactive)"
    )]
    unsafe_password: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Display your current refresh token")]
struct Display {}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers groups management commands")]
struct Groups {
    #[structopt(subcommand)]
    groups_command: GroupsCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers invites management commands")]
struct Invites {
    #[structopt(subcommand)]
    invites_command: InvitesCommand,
}

#[derive(StructOpt, Debug)]
struct Policies {
    #[structopt(subcommand)]
    policies_command: PoliciesCommand,
}

#[derive(StructOpt, Debug)]
enum PoliciesCommand {
    Create(CreatePolicy),
    Update(UpdatePolicy),
    Get(GetPolicy),
    Delete(DeletePolicy),
    List(ListPolicies),
}

#[derive(StructOpt, Debug)]
struct CreatePolicy {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the policy will relate to")]
    org_id: OrgId,
    #[structopt(long, short, help = "Policy's name")]
    name: String,
    #[structopt(long, short, help = "Policy's subjects")]
    subjects: Vec<String>,
    #[structopt(long, short, help = "Policy's resources")]
    resources: Vec<String>,
    #[structopt(long, short, help = "Policy's actions")]
    actions: Vec<String>,
    #[structopt(long, short, help = "Policy's effect")]
    effect: String,
}

#[derive(StructOpt, Debug)]
struct UpdatePolicy {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the policy is related to")]
    org_id: OrgId,
    #[structopt(long, short, parse(try_from_str = parse_policy_id), help = "Policy's id")]
    policy: esc_api::PolicyId,
    #[structopt(long, short, help = "Policy's name")]
    name: Option<String>,
    #[structopt(long, short, help = "Policy's subjects")]
    subjects: Option<Vec<String>>,
    #[structopt(long, short, help = "Policy's resources")]
    resources: Option<Vec<String>>,
    #[structopt(long, short, help = "Policy's actions")]
    actions: Option<Vec<String>>,
    #[structopt(long, short, help = "Policy's effect")]
    effect: Option<String>,
}

#[derive(StructOpt, Debug)]
struct GetPolicy {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the policy is related to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_policy_id), help = "Policy's id")]
    policy: esc_api::PolicyId,
}

#[derive(StructOpt, Debug)]
struct DeletePolicy {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the policy is related to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_policy_id), help = "Policy's id")]
    policy: esc_api::PolicyId,
}

#[derive(StructOpt, Debug)]
struct ListPolicies {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the policy is related to")]
    org_id: OrgId,
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
#[structopt(about = "Create a group")]
struct CreateGroup {
    #[structopt(long, short, help = "The group's name")]
    name: String,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the group will relate to")]
    org_id: OrgId,

    #[structopt(long, short, help = "The members of the group")]
    members: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Update a group")]
struct UpdateGroup {
    #[structopt(long, short, parse(try_from_str = parse_group_id), help = "The group's id")]
    id: GroupId,

    #[structopt(long, short, help = "The group's name")]
    name: Option<String>,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the group will relate to")]
    org_id: OrgId,

    #[structopt(long, short, help = "The members of the group")]
    members: Option<Vec<String>>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read a group information")]
struct GetGroup {
    #[structopt(long, short, parse(try_from_str = parse_group_id))]
    id: GroupId,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete a group")]
struct DeleteGroup {
    #[structopt(long, short, parse(try_from_str = parse_group_id), help = "The group's id")]
    id: GroupId,

    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the group will relate to")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "List groups")]
struct ListGroups {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the groups relate to")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
enum InvitesCommand {
    Create(CreateInvite),
    Update(UpdateInvite),
    Get(GetInvite),
    Delete(DeleteInvite),
    List(ListInvites),
}

#[derive(StructOpt, Debug)]
struct CreateInvite {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the invite will relate to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_email), help = "The email that will receive the invite")]
    email: esc_api::Email,

    #[structopt(long, short, parse(try_from_str = parse_group_id), help = "Group(s) the invite will associate the member with after confirmation")]
    group: Option<Vec<GroupId>>,
}

#[derive(StructOpt, Debug)]
struct UpdateInvite {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the invite will relate to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_invite_id), help = "The invite's id")]
    id: esc_api::InviteId,

    #[structopt(long, short, parse(try_from_str = parse_email), help = "The email that will receive the invite")]
    email: esc_api::Email,
}

#[derive(StructOpt, Debug)]
struct GetInvite {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the invite relates to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_invite_id), help = "The invite's id")]
    id: esc_api::InviteId,
}

#[derive(StructOpt, Debug)]
struct DeleteInvite {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the invite relates to")]
    org_id: OrgId,

    #[structopt(long, short, parse(try_from_str = parse_invite_id), help = "The invite's id")]
    id: esc_api::InviteId,
}

#[derive(StructOpt, Debug)]
struct ListInvites {
    #[structopt(long, short, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the invites relate to")]
    org_id: OrgId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers networks and peering management commands")]
struct Infra {
    #[structopt(subcommand)]
    infra_command: InfraCommand,
}

#[derive(StructOpt, Debug)]
enum InfraCommand {
    Networks(crate::apis::infra::Networks),
    Peerings(crate::apis::infra::Peerings),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers networks management commands")]
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
#[structopt(about = "Create a network")]
struct CreateNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the network will relate to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the network will relate to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_provider), help = "The cloud provider: aws, gcp or azure")]
    provider: esc_api::Provider,

    #[structopt(long, parse(try_from_str = parse_cidr), help = "Classless Inter-Domain Routing block (CIDR)")]
    cidr_block: cidr::Ipv4Cidr,

    #[structopt(long, help = "Human-readable description of the network")]
    description: String,

    #[structopt(long, help = "Cloud provider region")]
    region: String,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete a network")]
struct DeleteNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the network relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the network relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id), help = "A network's id")]
    id: esc_api::NetworkId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read a network information")]
struct GetNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the network relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the network relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id), help = "A network's id")]
    id: esc_api::NetworkId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "List networks of an organization, given a project")]
struct ListNetworks {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the networks relate to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the networks relate to")]
    project_id: esc_api::ProjectId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Update network")]
struct UpdateNetwork {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the network relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the network relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_network_id), help = "A network's id")]
    id: esc_api::NetworkId,

    #[structopt(long, help = "A human-readable network's description")]
    description: String,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Gathers peering management commands")]
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
#[structopt(about = "Create a peering")]
struct CreatePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the peering will relate to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the peering will relate to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_network_id), default_value = "", help = "The network id the peering will relate to")]
    network_id: esc_api::NetworkId,

    #[structopt(long, help = "Your cloud provider account id")]
    peer_account_id: String,

    #[structopt(long, help = "Your cloud provider network id")]
    peer_network_id: String,

    #[structopt(long, help = "Human-readable description for your peering")]
    description: String,

    #[structopt(long, help = "Your cloud provider network region")]
    peer_network_region: String,

    #[structopt(long, help = "Your network routes")]
    routes: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete a peering")]
struct DeletePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the peering relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the peering relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id), help = "The peering's id")]
    id: esc_api::PeeringId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Read a peering information")]
struct GetPeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the peering relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the peering relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id), help = "The peering's id")]
    id: esc_api::PeeringId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "List all peering related an organization, given a project id")]
struct ListPeerings {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the peerings relate to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the peerings relate to")]
    project_id: esc_api::ProjectId,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Update a peering")]
struct UpdatePeering {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the peering relates to")]
    org_id: esc_api::OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the peering relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_peering_id), help = "The peering's id")]
    id: esc_api::PeeringId,

    #[structopt(long, help = "A human-readable description for your peering")]
    description: String,
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


#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers cluster management commands")]
struct Mesdb {
    #[structopt(subcommand)]
    mesdb_command: MesdbCommand,
}

#[derive(Debug, StructOpt)]
enum MesdbCommand {
    Clusters(Clusters),
    Backups(Backups),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers cluster management commands")]
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
    Expand(ExpandCluster),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Create a cluster")]
struct CreateCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the cluster will relate to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the cluster will relate to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_network_id), help = "The network id the cluster will be set on")]
    network_id: esc_api::NetworkId,

    #[structopt(long, help = "A human-readable description of the cluster")]
    description: String,

    #[structopt(long, parse(try_from_str = parse_topology), help = "Either single-node or three-node-multi-zone")]
    topology: esc_api::Topology,

    #[structopt(
        long,
        help = "Type of instance, based on its hardware. For example, it could be F1 for a micro or C4 for a large instance"
    )]
    instance_type: String,

    #[structopt(long, help = "Total disk capacity in Gigabytes (GB)")]
    disk_size_in_gb: usize,

    #[structopt(
        long,
        help = "Type of disk. For example, if you are using AWS as a provider, it could be GP2"
    )]
    disk_type: String,

    #[structopt(long, help = "EventStoreDB server version")]
    server_version: String,

    #[structopt(
        long,
        parse(try_from_str = parse_projection_level),
        help = "The projection level of your database. Can be off, system or user "
    )]
    projection_level: esc_api::ProjectionLevel,

    #[structopt(long, help = "Optional id of backup to restore")]
    source_backup_id: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Get a cluster information")]
struct GetCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the cluster relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the cluster relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id), help = "Cluster's id")]
    id: esc_api::ClusterId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "List all clusters of an organization, given a project id")]
struct ListClusters {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "An organization's id")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "An project id that belongs to an organization pointed by --org-id")]
    project_id: esc_api::ProjectId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Update a cluster")]
struct UpdateCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the cluster relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the cluster relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id), help = "Id of the cluster you want to update")]
    id: esc_api::ClusterId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Delete a cluster")]
struct DeleteCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the cluster relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the cluster relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id), help = "Id of the cluster you want to delete")]
    id: esc_api::ClusterId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Expand a cluster")]
struct ExpandCluster {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the cluster relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the cluster relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_cluster_id), help = "Id of the cluster you want to expand")]
    id: esc_api::ClusterId,

    #[structopt(long, help = "Disk size in GB")]
    disk_size_in_gb: usize,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers backup management commands")]
struct Backups {
    #[structopt(subcommand)]
    backups_command: BackupsCommand,
}

#[derive(Debug, StructOpt)]
enum BackupsCommand {
    Create(CreateBackup),
    Get(GetBackup),
    List(ListBackups),
    Delete(DeleteBackup),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Create a backup")]
struct CreateBackup {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the backup will relate to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the backup will relate to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, parse(try_from_str = parse_cluster_id), help = "The id of the cluster to create backup of")]
    source_cluster_id: esc_api::ClusterId,

    #[structopt(long, help = "A human-readable description of the backup")]
    description: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Get information about a single backup")]
struct GetBackup {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the backup relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the backup relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_backup_id), help = "Backup's id")]
    id: esc_api::BackupId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "List all backups of an organization, given a project id")]
struct ListBackups {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "An organization's id")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "An project id that belongs to an organization pointed by --org-id")]
    project_id: esc_api::ProjectId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Delete a backup")]
struct DeleteBackup {
    #[structopt(long, parse(try_from_str = parse_org_id), default_value = "", help = "The organization id the backup relates to")]
    org_id: OrgId,

    #[structopt(long, parse(try_from_str = parse_project_id), default_value = "", help = "The project id the backup relates to")]
    project_id: esc_api::ProjectId,

    #[structopt(long, short, parse(try_from_str = parse_backup_id), help = "Id of the backup you want to delete")]
    id: esc_api::BackupId,
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

fn parse_backup_id(src: &str) -> Result<esc_api::BackupId, String> {
    Ok(esc_api::BackupId(src.to_string()))
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

    let client = Client::new(base_url, constants::ES_CLOUD_IDENTITY_URL.to_string())?;

    config::Settings::configure().await?;
    let mut store = TokenStore::new(&auth, client.tokens());
    store.configure().await?;

    if opt.debug {
        std::env::set_var("RUST_LOG", "esc_api=debug,esc=debug");
        env_logger::init();
    }

    match opt.cmd {
        Command::Access(access) => match access.access_command {
            AccessCommand::Groups(groups) => match groups.groups_command {
                GroupsCommand::Create(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let create_params = esc_api::command::groups::CreateGroupParams {
                        org_id: params.org_id,
                        name: params.name,
                        members: params.members,
                    };
                    let group_id = client.groups(&token).create(create_params).await?;

                    print_output(opt.render_in_json, group_id)?;
                }

                GroupsCommand::Update(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let mut update_group = client.groups(&token).update(params.id, params.org_id);

                    update_group.set_name(params.name);
                    update_group.set_members(params.members);
                    update_group.execute().await?;
                }

                GroupsCommand::Get(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let group_id_opt = client.groups(&token).get(params.id, params.org_id).await?;

                    match group_id_opt {
                        Some(group) => {
                            print_output(opt.render_in_json, group)?;
                        }

                        _ => {
                            eprintln!("Group doesn't exists");
                            std::process::exit(-1);
                        }
                    }
                }

                GroupsCommand::Delete(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    client
                        .groups(&token)
                        .delete(params.id, params.org_id)
                        .await?;
                }

                GroupsCommand::List(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let groups = client.groups(&token).list(params.org_id).await?;

                    print_output(opt.render_in_json, List(groups))?;
                }
            },

            AccessCommand::Invites(invites) => match invites.invites_command {
                InvitesCommand::Create(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let invite_id = client
                        .invites(&token)
                        .create(params.org_id, params.email, params.group)
                        .await?;

                    println!("{}", invite_id)
                }

                InvitesCommand::Update(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    client
                        .invites(&token)
                        .update(params.org_id, params.id, params.email)
                        .await?;
                }

                InvitesCommand::Get(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let invite = client.invites(&token).get(params.org_id, params.id).await?;

                    if let Some(invite) = invite {
                        print_output(opt.render_in_json, invite)?;
                    } else {
                        std::process::exit(-1);
                    }
                }

                InvitesCommand::Delete(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    client
                        .invites(&token)
                        .delete(params.org_id, params.id)
                        .await?;
                }

                InvitesCommand::List(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let invites = client.invites(&token).list(params.org_id).await?;

                    print_output(opt.render_in_json, List(invites))?;
                }
            },

            AccessCommand::Tokens(tokens) => match tokens.tokens_command {
                TokensCommand::Create(params) => {
                    let password = if let Some(passw) = params.unsafe_password {
                        Ok(passw)
                    } else {
                        rpassword::read_password_from_tty(Some("Password: "))
                    }?;

                    let audience = TokenStore::build_audience_str(&auth.audience);

                    let token = client
                        .tokens()
                        .create(
                            &auth.id,
                            params.email.as_str(),
                            password.as_str(),
                            audience.as_str(),
                        )
                        .await?;

                    let refresh = client
                        .tokens()
                        .refresh(&auth.id, token.refresh_token().unwrap().as_str())
                        .await?;

                    let token = token.update_access_token(refresh.access_token());
                    let new_token_bytes = serde_json::to_vec(&token)?;

                    let token_path = TokenStore::token_dirs()
                        .join(auth.audience.host().expect("We have a host in this URI"));

                    tokio::fs::write(&token_path, &new_token_bytes).await?;

                    println!("Token created for audience {}", audience.as_str());
                    println!("{}", token.refresh_token().unwrap().as_str());
                }
                TokensCommand::Display(_params) => {
                    let token = store.active_token().await?;
                    if let Some(token) = token {
                        println!("{}", token.refresh_token().unwrap());
                    } else {
                        println!("No active refresh token");
                    }
                }
            },

            AccessCommand::Policies(policies) => match policies.policies_command {
                PoliciesCommand::Create(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let create_params = esc_api::command::policies::CreatePolicyParams {
                        name: params.name,
                        subjects: params.subjects,
                        resources: params.resources,
                        actions: params.actions,
                        effect: params.effect,
                    };
                    let id = client
                        .policies(&token)
                        .create(params.org_id, create_params)
                        .await?;

                    print_output(opt.render_in_json, id)?;
                }

                PoliciesCommand::Update(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let update_params = esc_api::command::policies::UpdatePolicyParams {
                        name: params.name,
                        subjects: params.subjects,
                        resources: params.resources,
                        actions: params.actions,
                        effect: params.effect,
                    };

                    client
                        .policies(&token)
                        .update(params.org_id, params.policy, update_params)
                        .await?;
                }

                PoliciesCommand::Delete(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    client
                        .policies(&token)
                        .delete(params.org_id, params.policy)
                        .await?;
                }

                PoliciesCommand::Get(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let policy = client
                        .policies(&token)
                        .get(params.org_id, params.policy)
                        .await?;

                    print_output(opt.render_in_json, policy)?;
                }

                PoliciesCommand::List(params) => {
                    let token = store.access(opt.refresh_token).await?;
                    let policies = client.policies(&token).list(params.org_id).await?;

                    print_output(opt.render_in_json, List(policies))?;
                }
            },
        },

        Command::Infra(infra) => match infra.command {
            crate::apis::infra::InfraCommand::Networks(networks) => {
                let cc = CliConfig{
                    client: client.clone(),
                    token: store.access(opt.refresh_token).await?,
                    render_in_json: opt.render_in_json,
                };
                networks.command.exec(&cc).await?
            },

            crate::apis::infra::InfraCommand::Peerings(peerings) => {
                let cc = CliConfig{
                    client: client.clone(),
                    token: store.access(opt.refresh_token).await?,
                    render_in_json: opt.render_in_json,
                };
                peerings.command.exec(&cc).await?
            },
        },

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

        Command::Resources(res) => match res.command {
            crate::apis::resources::ResourcesCommand::Organizations(orgs) => {
                let cc = CliConfig{
                    client: client.clone(),
                    token: store.access(opt.refresh_token).await?,
                    render_in_json: opt.render_in_json,
                };
                orgs.command.exec(&cc).await?               
            },

            crate::apis::resources::ResourcesCommand::Projects(projs) => {
                let cc = CliConfig{
                    client: client.clone(),
                    token: store.access(opt.refresh_token).await?,
                    render_in_json: opt.render_in_json,
                };
                projs.command.exec(&cc).await?                
            },
        },

        Command::Mesdb(mesdb) => {
            let token = store.access(opt.refresh_token).await?;
            match mesdb.mesdb_command {
                MesdbCommand::Clusters(clusters) => match clusters.clusters_command {
                    ClustersCommand::Create(params) => {
                        let create_params = esc_api::command::clusters::CreateClusterParams {
                            network_id: params.network_id,
                            description: params.description,
                            topology: params.topology,
                            instance_type: params.instance_type,
                            disk_size_gb: params.disk_size_in_gb,
                            disk_type: params.disk_type,
                            server_version: params.server_version,
                            projection_level: params.projection_level,
                            source_backup_id: params.source_backup_id,
                        };
                        let cluster_id = client
                            .clusters(&token)
                            .create(params.org_id, params.project_id, create_params)
                            .await?;

                        print_output(opt.render_in_json, cluster_id)?;
                    }

                    ClustersCommand::Get(params) => {
                        let cluster = client
                            .clusters(&token)
                            .get(params.org_id, params.project_id, params.id)
                            .await?;

                        print_output(opt.render_in_json, enrich::enrich_cluster(cluster))?;
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

                        print_output(
                            opt.render_in_json,
                            List(clusters.into_iter().map(enrich::enrich_cluster).collect()),
                        )?;
                    }

                    ClustersCommand::Expand(params) => {
                        client
                            .clusters(&token)
                            .expand(
                                params.org_id,
                                params.project_id,
                                params.id,
                                params.disk_size_in_gb,
                            )
                            .await?;
                    }
                },
                MesdbCommand::Backups(clusters) => match clusters.backups_command {
                    BackupsCommand::Create(params) => {
                        let create_params = esc_api::command::backups::CreateBackupParams {
                            source_cluster_id: params.source_cluster_id,
                            description: params.description,
                        };
                        let backup_id = client
                            .backups(&token)
                            .create(params.org_id, params.project_id, create_params)
                            .await?;

                        print_output(opt.render_in_json, backup_id)?;
                    }

                    BackupsCommand::Get(params) => {
                        let backup = client
                            .backups(&token)
                            .get(params.org_id, params.project_id, params.id)
                            .await?;

                        print_output(opt.render_in_json, backup)?;
                    }

                    BackupsCommand::Delete(params) => {
                        client
                            .backups(&token)
                            .delete(params.org_id, params.project_id, params.id)
                            .await?;
                    }

                    BackupsCommand::List(params) => {
                        let backups = client
                            .backups(&token)
                            .list(params.org_id, params.project_id)
                            .await?;

                        print_output(opt.render_in_json, List(backups))?;
                    }
                },
            }
        }

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
