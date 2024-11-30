mod copy_button;
mod format_button;
mod header;
mod main_input;
mod state;

use leptos::{config::LeptosOptions, prelude::*};
use leptos_meta::*;
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

use self::{
  copy_button::CopyButton, format_button::FormatButton, header::Header,
  main_input::MainInput, state::StateProvider,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <meta name="description" content="A simple JSON prettifier, written in Rust."/>
        <meta name="theme-color" content="#1c1c1c"/>

        <AutoReload options=options.clone()/>
        <HydrationScripts options=options.clone() islands=true/>
        <HashedStylesheet options=options.clone() id="leptos"/>
        <link rel="shortcut icon" type="image/ico" href="/favicon.png"/>

        <Title text="JSON Prettifier"/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Router>
      <Routes fallback=|| "Page not found.".into_view()>
        <Route path=path!("/") view=HomePage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <main class="w-screen h-screen">
      <Header/>
      <StateProvider>
        <div class="relative w-full h-[calc(100%-2.5rem-2px)] p-4">
          <MainInput />
          <div class="absolute right-8 bottom-8 flex flex-row gap-2">
            <CopyButton />
            <FormatButton />
          </div>
        </div>
      </StateProvider>
    </main>
  }
}

fn format_json_string(json_string: &str) -> Result<String, String> {
  let json_value = serde_json::from_str::<serde_json::Value>(json_string)
    .map_err(|e| e.to_string())?;
  serde_json::to_string_pretty(&json_value).map_err(|e| e.to_string())
}
