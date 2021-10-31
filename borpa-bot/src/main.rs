mod commands;
mod config;
mod constants;

use std::{error::Error, sync::Arc};

use borpa_commands::{command::CommandKind, handler::Handler};
use futures::StreamExt;
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::info;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use twilight_http::Client as HttpClient;
use twilight_model::application::command::{CommandOption, OptionsCommandOptionData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    PrometheusBuilder::new()
        .listen_address(constants::CONFIG.metrics.address)
        .install()?;

    let scheme = ShardScheme::Auto;

    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES;

    let (cluster, mut events) = Cluster::builder(&constants::CONFIG.bot.token, intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    let cluster = Arc::new(cluster);

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = Arc::new(
        HttpClient::builder()
            .token(constants::CONFIG.bot.token.to_owned())
            .build(),
    );

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    let handler = Handler::new(constants::COMMANDS.clone());

    // Vector of root commands that are already registered
    //let mut grouped = vec![];

    // for (name, command) in &handler.commands {
    //     println!("START COMMAND: {}", name);
    //     let options: Vec<CommandOption> = match &command.kind {
    //         CommandKind::Command => {
    //             if grouped.contains(&command.name) {
    //                 println!("SKIPPING {}", name);
    //                 continue;
    //             }

    //             grouped.push(command.name.to_owned());
    //             command.options.to_owned()
    //         }
    //         // c is our subcommandgroup
    //         CommandKind::Subcommand(c) => {
    //             // c_ is the root command
    //             if let CommandKind::SubcommandGroup(c_) = &c.kind {
    //                 if grouped.contains(&c_.name) {
    //                     println!("SKIPPING {}", name);
    //                     continue;
    //                 }

    //                 // Now we have a vector with our subcommand in it
    //                 let subcommand = CommandOption::SubCommand(OptionsCommandOptionData {
    //                     name: name.to_string(),
    //                     description: command.description.to_owned(),
    //                     options: command.options.to_owned(),
    //                 });

    //                 let mut merged = vec![subcommand];
    //                 merged.extend(c.options.to_owned());

    //                 let subcommandgroup =
    //                     CommandOption::SubCommandGroup(OptionsCommandOptionData {
    //                         name: c.name.to_string(),
    //                         description: c.description.to_owned(),
    //                         options: merged,
    //                     });

    //                 let mut merged = vec![subcommandgroup];
    //                 merged.extend(c_.options.to_owned());

    //                 grouped.push(c.name.to_string());

    //                 merged
    //             } else {
    //                 vec![]
    //             }
    //         }
    //         CommandKind::SubcommandGroup(c) => {
    //             if grouped.contains(&c.name) {
    //                 println!("SKIPPING {}", name);
    //                 continue;
    //             }

    //             let subcommandgroup = CommandOption::SubCommandGroup(OptionsCommandOptionData {
    //                 name: c.name.to_string(),
    //                 description: c.description.to_owned(),
    //                 options: command.options.clone(),
    //             });

    //             let mut merged = vec![subcommandgroup];
    //             merged.extend(c.options.to_owned());

    //             grouped.push(c.name.to_string());

    //             merged
    //         }
    //     };
    //     println!("END COMMAND");
    // }

    let cmd = handler.find_command("owner".to_string());

    if let Some(c) = cmd {
        println!(
            "Found {} command with description {}",
            c.name, c.description
        );
        //let h = &c.handler;
        //h();
    } else {
        println!("Command not found");
    }

    while let Some((shard_id, event)) = events.next().await {
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, http.clone()));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::ShardConnected(_) => {
            info!("Connected on shard {}", shard_id);
        }
        _ => {}
    }

    Ok(())
}
