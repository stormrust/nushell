#[macro_use]
pub(crate) mod macros;
pub(crate) mod append;
pub mod autoenv;
pub(crate) mod autoenv_trust;
pub(crate) mod autoenv_untrust;
pub(crate) mod autoview;
pub(crate) mod classified;
pub mod command;
pub(crate) mod debug;
pub(crate) mod def;
pub(crate) mod default;

pub mod default_context;

pub(crate) mod echo;
pub(crate) mod exit;
pub(crate) mod flatten;
pub(crate) mod get;
pub(crate) mod help;
pub(crate) mod history;
pub(crate) mod let_;
pub(crate) mod let_env;
pub(crate) mod ls;
pub(crate) mod nu;
pub(crate) mod pivot;
pub(crate) mod run_external;
pub(crate) mod source;
pub(crate) mod table;
pub(crate) mod uniq;
pub(crate) mod version;
pub(crate) mod with_env;

pub(crate) use autoenv::Autoenv;
pub(crate) use autoenv_trust::AutoenvTrust;
pub(crate) use autoenv_untrust::AutoenvUnTrust;
pub(crate) use autoview::Autoview;
pub(crate) use debug::Debug;
pub(crate) use def::Def;
pub(crate) use echo::Echo;
pub(crate) use nu::NuPlugin;
pub(crate) mod clear;
pub(crate) use clear::Clear;
pub(crate) mod touch;
pub(crate) use exit::Exit;
pub(crate) use flatten::Command as Flatten;
pub(crate) use get::Command as Get;
pub(crate) use help::Help;
pub(crate) use history::History;
pub(crate) use let_::Let;
pub(crate) use let_env::LetEnv;
pub(crate) use ls::Ls;
pub(crate) use pivot::Pivot;
pub(crate) use run_external::RunExternalCommand;
pub(crate) use source::Source;
pub(crate) use table::Table;
pub(crate) use touch::Touch;
pub(crate) use uniq::Uniq;
pub(crate) use version::Version;
pub(crate) use with_env::WithEnv;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::{test_anchors, test_examples};
    use nu_engine::{whole_stream_command, Command};
    use nu_errors::ShellError;

    fn full_tests() -> Vec<Command> {
        vec![
            whole_stream_command(Append),
            whole_stream_command(GroupBy),
            whole_stream_command(Insert),
            whole_stream_command(Move),
            whole_stream_command(Update),
            whole_stream_command(Empty),
            // whole_stream_command(Select),
            // whole_stream_command(Get),
            // Str Command Suite
            whole_stream_command(Str),
            whole_stream_command(StrToDecimal),
            whole_stream_command(StrToInteger),
            whole_stream_command(StrDowncase),
            whole_stream_command(StrUpcase),
            whole_stream_command(StrCapitalize),
            whole_stream_command(StrFindReplace),
            whole_stream_command(StrFrom),
            whole_stream_command(StrSubstring),
            whole_stream_command(StrToDatetime),
            whole_stream_command(StrContains),
            whole_stream_command(StrIndexOf),
            whole_stream_command(StrTrim),
            whole_stream_command(StrTrimLeft),
            whole_stream_command(StrTrimRight),
            whole_stream_command(StrStartsWith),
            whole_stream_command(StrEndsWith),
            //whole_stream_command(StrCollect),
            whole_stream_command(StrLength),
            whole_stream_command(StrLPad),
            whole_stream_command(StrReverse),
            whole_stream_command(StrRPad),
            whole_stream_command(StrCamelCase),
            whole_stream_command(StrPascalCase),
            whole_stream_command(StrKebabCase),
            whole_stream_command(StrSnakeCase),
            whole_stream_command(StrScreamingSnakeCase),
            whole_stream_command(ToMarkdown),
        ]
    }

    fn only_examples() -> Vec<Command> {
        let mut commands = full_tests();
        commands.extend(vec![whole_stream_command(Flatten)]);
        commands
    }

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        for cmd in only_examples() {
            test_examples(cmd)?;
        }

        Ok(())
    }

    #[test]
    fn tracks_metadata() -> Result<(), ShellError> {
        for cmd in full_tests() {
            test_anchors(cmd)?;
        }

        Ok(())
    }
}
