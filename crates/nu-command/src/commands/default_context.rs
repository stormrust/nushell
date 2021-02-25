use crate::prelude::*;
use nu_engine::basic_evaluation_context;
use nu_engine::whole_stream_command;
use std::error::Error;

pub fn create_default_context(interactive: bool) -> Result<EvaluationContext, Box<dyn Error>> {
    let context = basic_evaluation_context()?;

    {
        use crate::commands::*;

        context.add_commands(vec![
            // Fundamentals
            whole_stream_command(NuPlugin),
            whole_stream_command(Let),
            whole_stream_command(LetEnv),
            whole_stream_command(Def),
            whole_stream_command(Source),
            // System/file operations
            whole_stream_command(Ls),
            whole_stream_command(Help),
            whole_stream_command(History),
            whole_stream_command(Touch),
            whole_stream_command(Version),
            whole_stream_command(Clear),
            whole_stream_command(Debug),
            whole_stream_command(WithEnv),
            // Viewers
            whole_stream_command(Autoview),
            whole_stream_command(Table),
            // Text manipulation
            whole_stream_command(Echo),
            // Column manipulation
            whole_stream_command(Get),
            // Row manipulation
            whole_stream_command(Uniq),
            // Shells
            whole_stream_command(Exit),
            // Table manipulation
            whole_stream_command(Flatten),
            whole_stream_command(Pivot),
            // Data processing
            whole_stream_command(Autoenv),
            whole_stream_command(AutoenvTrust),
            whole_stream_command(AutoenvUnTrust),
            // "Private" commands (not intended to be accessed directly)
            whole_stream_command(RunExternalCommand { interactive }),
        ]);
    }

    Ok(context)
}
