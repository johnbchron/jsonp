use axum::Router;
use color_eyre::eyre::Result;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use site_app::{shell, App};
use tower_http::compression::CompressionLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install().expect("Failed to install color_eyre");

  let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or(tracing_subscriber::EnvFilter::new("info"));
  let fmt_layer = tracing_subscriber::fmt::layer()
    .with_target(false)
    .with_writer(std::io::stderr);
  let registry = tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer);

  #[cfg(not(feature = "chrome-tracing"))]
  {
    registry.init();
  }
  #[cfg(feature = "chrome-tracing")]
  let guard = {
    let (chrome_layer, guard) =
      tracing_chrome::ChromeLayerBuilder::new().build();
    registry.with(chrome_layer).try_init()?;
    guard
  };

  // Setting get_configuration(None) means we'll be using cargo-leptos's env
  // values For deployment these variables are:
  // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
  // Alternately a file can be specified such as Some("Cargo.toml")
  // The file would need to be included with the executable when moved to
  // deployment
  let conf = get_configuration(None).unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let routes = generate_route_list(App);

  // build our application with a route
  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .layer(CompressionLayer::new())
    .with_state(leptos_options);

  let listener = tokio::net::TcpListener::bind(&addr).await?;

  tracing::info!("listening on http://{}", &addr);

  tokio::spawn(async move { axum::serve(listener, app).await });

  tokio::signal::ctrl_c().await?;
  tracing::info!("shutting down");

  #[cfg(feature = "chrome-tracing")]
  {
    guard.flush();
    tracing::info!("chrome tracing data written");
  }

  Ok(())
}
