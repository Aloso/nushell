use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::prelude::*;
use std::path::PathBuf;

pub struct LS;

#[derive(Deserialize)]
pub struct LsArgs {
    path: Option<Tagged<PathBuf>>,
    full: bool,
}

impl WholeStreamCommand for LS {
    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> Signature {
        Signature::build("ls")
            .optional(
                "path",
                SyntaxShape::Pattern,
                "a path to get the directory contents from",
            )
            .switch("full", "list all available columns for each entry")
    }

    fn usage(&self) -> &str {
        "View the contents of the current or given path."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        args.process(registry, ls)?.run()
        // ls(args, registry)
    }
}

fn ls(LsArgs { path, full }: LsArgs, context: RunnableContext) -> Result<OutputStream, ShellError> {
    context.shell_manager.ls(path, &context, full)
}
