use crate::prelude::*;
use nu_engine::evaluate_baseline_expr;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{
    hir::CapturedBlock,
    hir::{ClassifiedCommand, SpannedExpression},
    Signature, SyntaxShape, Value,
};

pub struct Command;

#[derive(Deserialize)]
pub struct Arguments {
    block: CapturedBlock,
}

impl WholeStreamCommand for Command {
    fn name(&self) -> &str {
        "where"
    }

    fn signature(&self) -> Signature {
        Signature::build("where").required(
            "condition",
            SyntaxShape::RowCondition,
            "the condition that must match",
        )
    }

    fn usage(&self) -> &str {
        "Filter table to match the condition."
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        where_command(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "List all files in the current directory with sizes greater than 2kb",
                example: "ls | where size > 2kb",
                result: None,
            },
            Example {
                description: "List only the files in the current directory",
                example: "ls | where type == File",
                result: None,
            },
            Example {
                description: "List all files with names that contain \"Car\"",
                example: "ls | where name =~ \"Car\"",
                result: None,
            },
            Example {
                description: "List all files that were modified in the last two weeks",
                example: "ls | where modified <= 2wk",
                result: None,
            },
        ]
    }
}
fn where_command(raw_args: CommandArgs) -> Result<OutputStream, ShellError> {
    let context = Arc::new(EvaluationContext::from_args(&raw_args));
    let tag = raw_args.call_info.name_tag.clone();
    let (Arguments { block }, input) = raw_args.process()?;
    let condition = {
        if block.block.block.len() != 1 {
            return Err(ShellError::labeled_error(
                "Expected a condition",
                "expected a condition",
                tag,
            ));
        }
        match block.block.block[0].pipelines.get(0) {
            Some(item) => match item.list.get(0) {
                Some(ClassifiedCommand::Expr(expr)) => expr.clone(),
                _ => {
                    return Err(ShellError::labeled_error(
                        "Expected a condition",
                        "expected a condition",
                        tag,
                    ));
                }
            },
            None => {
                return Err(ShellError::labeled_error(
                    "Expected a condition",
                    "expected a condition",
                    tag,
                ));
            }
        }
    };

    Ok(WhereIterator {
        condition,
        context,
        input,
        block,
    }
    .to_output_stream())
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::ShellError;

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        use crate::examples::test as test_examples;

        test_examples(Command {})
    }
}

struct WhereIterator {
    condition: Box<SpannedExpression>,
    context: Arc<EvaluationContext>,
    input: InputStream,
    block: CapturedBlock,
}

impl Iterator for WhereIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.input.next() {
            self.context.scope.enter_scope();
            self.context.scope.add_vars(&self.block.captured.entries);

            if let Some((arg, _)) = self.block.block.params.positional.first() {
                self.context.scope.add_var(arg.name(), x.clone());
            }

            //FIXME: should we use the scope that's brought in as well?
            let condition = evaluate_baseline_expr(&self.condition, &self.context);
            self.context.scope.exit_scope();

            match condition {
                Ok(condition) => match condition.as_bool() {
                    Ok(b) => {
                        if b {
                            return Some(x);
                        }
                    }
                    Err(e) => return Some(Value::error(e)),
                },
                Err(e) => return Some(Value::error(e)),
            }
        }

        None
    }
}
