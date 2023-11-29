mod children_proc;
mod parent_proc;
mod args;

use crate::children_proc::children_proc;
use crate::parent_proc::parent_proc;
use crate::args::Args;
use std::{env, error::Error};
use nix::unistd::{fork, ForkResult};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::try_from(env::args().collect::<Vec<_>>())?;

    let fork_result = unsafe {fork()}?;
    match fork_result {
        ForkResult::Child => {
            children_proc(args.app, args.args)?;
        },
        ForkResult::Parent { child } => {
            parent_proc(child)?;
        }
    }
    Ok(())
}
