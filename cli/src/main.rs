use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Access(Access),
}

#[derive(StructOpt, Debug)]
struct Access {
    #[structopt(subcommand)]
    access_command: AccessCommand,
}

#[derive(StructOpt, Debug)]
enum AccessCommand {
    User(User),
}

#[derive(StructOpt, Debug)]
struct User {
    #[structopt(subcommand)]
    user_command: UserCommand,
}

#[derive(StructOpt, Debug)]
enum UserCommand {
    List,
    Revoke,
}

fn main() {
    let opt = Opt::from_args();
    println!("> {:?}", opt);
}
