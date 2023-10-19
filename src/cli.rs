use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Manage temporary Postgres instances
pub struct TopLevel {
    #[argh(subcommand)]
    pub subcommand: SubcommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubcommandEnum {
    New(New),
    Delete(Delete),
    Connect(Connect),
    Connstring(Connstring),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Crate a new temporary Postgres database
#[argh(subcommand, name = "new")]
pub struct New {
    #[argh(option)]
    /// the port to bind to
    pub port: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Delete the existing emporary Postgres database
#[argh(subcommand, name = "delete")]
pub struct Delete {}

#[derive(FromArgs, PartialEq, Debug)]
/// Connect to the existing emporary Postgres database
#[argh(subcommand, name = "connect")]
pub struct Connect {}

#[derive(FromArgs, PartialEq, Debug)]
/// Display the connstring to connect to the active Postgres database
#[argh(subcommand, name = "connstring")]
pub struct Connstring {}
