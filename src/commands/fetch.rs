use clap::Args;

use super::Exec;

#[derive(Debug, Args)]
pub struct Fetch {}

impl Exec for Fetch {
    fn exec(&self) -> anyhow::Result<()> {
        panic!("init is not implemented")
    }
}
