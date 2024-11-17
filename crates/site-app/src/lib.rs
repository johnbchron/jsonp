use leptos::{config::LeptosOptions, prelude::*};
use leptos_meta::*;
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <AutoReload options=options.clone()/>
        <HydrationScripts options=options.clone() islands=true/>
        <HashedStylesheet options=options.clone() id="leptos"/>
        <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

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
    <p>"Hello, World!"</p>

    <TestIsland />
  }
}

#[island]
pub fn TestIsland() -> impl IntoView {
  let (count, set_count) = signal(0);

  view! {
    <div>
      <p>"Count: " { count }</p>
      <button
        on:click={move |_| set_count(count.get() + 1)}
      >
        "Increment"
      </button>
    </div>
  }
}
