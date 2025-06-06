use clap::Args;

use super::Exec;

#[derive(Debug, Args)]
pub struct Push {}

impl Exec for Push {
    fn exec(&self) -> anyhow::Result<()> {
        panic!("init is not implemented")
    }
}
