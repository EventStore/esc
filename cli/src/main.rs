#[macro_use]
extern crate log;

mod constants;
mod store;

use crate::store::{Auth, TokenStore};
use esc_api::{Client, ClientId, GroupId, OrgId};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "esc",
    about = "EventStoreDB Cloud tool.",
    author = "Event Store Limited <ops@eventstore.com>"
)]
struct Opt {
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

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Access(Access),
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

    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    members: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct UpdateGroup {
    #[structopt(long, short)]
    id: String,

    #[structopt(long, short)]
    name: Option<String>,

    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    members: Option<Vec<String>>,
}

#[derive(StructOpt, Debug)]
struct DeleteGroup {
    #[structopt(long, short)]
    id: String,

    #[structopt(long, short)]
    org_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let audience = constants::ES_CLOUD_API_AUDIENCE.parse()?;
    let auth = Auth {
        id: ClientId(constants::ES_CLIENT_ID.to_owned()),
        username: opt.username,
        password: opt.password,
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

    match opt.cmd {
        Command::Access(access) => match access.access_command {
            AccessCommand::Groups(groups) => match groups.groups_command {
                GroupsCommand::Create(params) => {
                    let token = store.access().await?;
                    let group_id = client
                        .groups(&token)
                        .create(params.name, OrgId(params.org_id), params.members)
                        .await?;

                    println!("{}", group_id);
                }

                GroupsCommand::Update(params) => {
                    let token = store.access().await?;
                    let mut update_group = client
                        .groups(&token)
                        .update(GroupId(params.id), OrgId(params.org_id));

                    update_group.set_name(params.name);
                    update_group.set_members(params.members);

                    let group_id = update_group.execute().await?;

                    println!("group-id {} updated", group_id);
                }

                GroupsCommand::Delete(params) => {
                    let token = store.access().await?;
                    client
                        .groups(&token)
                        .delete(GroupId(params.id), OrgId(params.org_id))
                        .await?;

                    println!("group deleted");
                }
            },

            ignored => {
                println!("$> {:?}", ignored);
                unimplemented!()
            }
        },
    };

    Ok(())
}
