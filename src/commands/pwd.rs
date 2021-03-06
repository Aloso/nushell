use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::parser::registry::Signature;
use crate::prelude::*;

pub struct PWD;

impl WholeStreamCommand for PWD {
    fn name(&self) -> &str {
        "pwd"
    }

    fn signature(&self) -> Signature {
        Signature::build("pwd")
    }

    fn usage(&self) -> &str {
        "Output the current working directory."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        pwd(args, registry)
    }
}

pub fn pwd(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let shell_manager = args.shell_manager.clone();
    let args = args.evaluate_once(registry)?;
    shell_manager.pwd(args)
}
