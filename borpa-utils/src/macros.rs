#[macro_export]
macro_rules! command {
    ($name:literal, $description:literal, $options:expr, $type:expr, $kind:expr, $handler:expr) => {{
        std::sync::Arc::new(borpa_commands::command::Command {
            name: String::from($name),
            description: String::from($description),
            options: $options,
            r#type: $type,
            kind: $kind,
            handler: Box::new(move |ctx| Box::pin($handler(ctx))),
        })
    }};
}
