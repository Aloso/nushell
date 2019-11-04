use crate::commands::WholeStreamCommand;
use crate::data::Value;
use crate::errors::ShellError;
use crate::parser::hir::path::{PathMember, RawPathMember};
use crate::prelude::*;
use crate::utils::did_you_mean;
use log::trace;

pub struct Get;

#[derive(Deserialize)]
pub struct GetArgs {
    member: ColumnPath,
    rest: Vec<ColumnPath>,
}

impl WholeStreamCommand for Get {
    fn name(&self) -> &str {
        "get"
    }

    fn signature(&self) -> Signature {
        Signature::build("get")
            .required(
                "member",
                SyntaxShape::ColumnPath,
                "the path to the data to get",
            )
            .rest(
                SyntaxShape::ColumnPath,
                "optionally return additional data by path",
            )
    }

    fn usage(&self) -> &str {
        "Open given cells as text."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        args.process(registry, get)?.run()
    }
}

pub type ColumnPath = Vec<PathMember>;

pub fn get_column_path(
    path: &ColumnPath,
    obj: &Tagged<Value>,
) -> Result<Tagged<Value>, ShellError> {
    let fields = path.clone();

    let value = obj.get_data_by_column_path(
        obj.tag(),
        path,
        Box::new(move |(obj_source, column_path_tried)| {
            match obj_source {
                Value::Table(rows) => {
                    let total = rows.len();
                    let end_tag = match fields.iter().nth_back(if fields.len() > 2 { 1 } else { 0 })
                    {
                        Some(last_field) => last_field.span(),
                        None => column_path_tried.span(),
                    };

                    return ShellError::labeled_error_with_secondary(
                        "Row not found",
                        format!("There isn't a row indexed at '{}'", **column_path_tried),
                        column_path_tried.span(),
                        format!("The table only has {} rows (0..{})", total, total - 1),
                        end_tag,
                    );
                }
                _ => {}
            }

            if let RawPathMember::String(name) = &column_path_tried.item {
                match did_you_mean(&obj_source, &name.clone().tagged(column_path_tried.span)) {
                    Some(suggestions) => {
                        return ShellError::labeled_error(
                            "Unknown column",
                            format!("did you mean '{}'?", suggestions[0].1),
                            span_for_spanned_list(fields.iter().map(|p| p.span())),
                        )
                    }
                    None => {}
                }
            }

            return ShellError::labeled_error(
                "Unknown column",
                "row does not contain this column",
                span_for_spanned_list(fields.iter().map(|p| p.span())),
            );
        }),
    );

    let res = match value {
        Ok(fetched) => match fetched {
            Some(Tagged { item: v, tag }) => Ok((v.clone()).tagged(&tag)),
            None => match obj {
                // If its None check for certain values.
                Tagged {
                    item: Value::Primitive(Primitive::String(_)),
                    ..
                } => Ok(obj.clone()),
                Tagged {
                    item: Value::Primitive(Primitive::Path(_)),
                    ..
                } => Ok(obj.clone()),
                _ => Ok(Value::nothing().tagged(&obj.tag)),
            },
        },
        Err(reason) => Err(reason),
    };

    res
}

pub fn get(
    GetArgs {
        member,
        rest: fields,
    }: GetArgs,
    RunnableContext { input, .. }: RunnableContext,
) -> Result<OutputStream, ShellError> {
    trace!("get {:?} {:?}", member, fields);

    let stream = input
        .values
        .map(move |item| {
            let mut result = VecDeque::new();

            let member = vec![member.clone()];

            let column_paths = vec![&member, &fields]
                .into_iter()
                .flatten()
                .collect::<Vec<&ColumnPath>>();

            for path in column_paths {
                let res = get_column_path(&path, &item);

                match res {
                    Ok(got) => match got {
                        Tagged {
                            item: Value::Table(rows),
                            ..
                        } => {
                            for item in rows {
                                result.push_back(ReturnSuccess::value(item.clone()));
                            }
                        }
                        other => result
                            .push_back(ReturnSuccess::value((*other).clone().tagged(&item.tag))),
                    },
                    Err(reason) => result.push_back(Err(reason)),
                }
            }
            result
        })
        .flatten();

    Ok(stream.to_output_stream())
}
