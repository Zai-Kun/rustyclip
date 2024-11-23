use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// A clipboard management tool
pub struct RustyBoard {
    #[argh(subcommand)]
    pub command: Commands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Commands {
    List(ListCommand),
    Store(StoreCommand),
    Get(GetCommand),
    Remove(RemoveCommand),
    Clear(ClearCommand),
}

#[derive(FromArgs, Debug)]
/// List all stored clipboard items
#[argh(subcommand, name = "list")]
pub struct ListCommand {}

#[derive(FromArgs, Debug)]
/// Store a new clipboard item
#[argh(subcommand, name = "store")]
pub struct StoreCommand {}

#[derive(FromArgs, Debug)]
/// Get a clipboard item by query
#[argh(subcommand, name = "get")]
pub struct GetCommand {
    #[argh(positional)]
    ///  it must start with an int (index to the item) sperated by ':' or just an int
    pub query: Option<String>,
}

#[derive(FromArgs, Debug)]
/// Remove a clipboard item by query
#[argh(subcommand, name = "remove")]
pub struct RemoveCommand {
    #[argh(positional)]
    ///  it must start with an int (index to the item) sperated by ':' or just an int
    pub query: Option<String>,
}

#[derive(FromArgs, Debug)]
/// Clear all clipboard items
#[argh(subcommand, name = "clear")]
pub struct ClearCommand {}

pub fn parse_args() -> RustyBoard {
    argh::from_env()
}
