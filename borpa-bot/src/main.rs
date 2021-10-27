mod commands;
mod config;
mod constants;

use std::{error::Error, sync::Arc, time::Duration};

use borpa_commands::{command::CommandKind, handler::Handler};
use futures::StreamExt;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_runtime::{exporters::LogExporter, observers::JsonBuilder, Receiver};
use tracing::{info, Level};
use tracing_log::log;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use twilight_http::Client as HttpClient;
use twilight_model::application::command::CommandOption;

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

    while let Some((shard_id, event)) = events.next().await {
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, http.clone()));
    }

    /*
    for (name, command) in &handler.commands {
        let options: Vec<CommandOption> = match &command.kind {
            CommandKind::Command =>
            CommandKind::Subcommand(c) => {
                if let CommandKind::SubcommandGroup(c_) = &c.kind {
                } else {
                }
            }
            CommandKind::SubcommandGroup(c) => {}
        };
    }
    */

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
