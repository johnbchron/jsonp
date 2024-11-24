mod header;

use leptos::{
  config::LeptosOptions, logging::log, prelude::*, task::spawn_local,
};
use leptos_meta::*;
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

use self::header::Header;

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
      <MainInput/>
    </main>
  }
}

#[island]
pub fn MainInput() -> impl IntoView {
  let textarea_class = "w-full h-full py-1 px-2 bg-backgroundSecondary \
                        text-sm text-content1 resize-none border \
                        border-gray-6 outline-none box-border font-mono \
                        relative";
  let placeholder = "Paste JSON here...";

  let (input_contents, set_input_contents) = signal(String::new());
  let sync_input_contents = move |new_contents: String| {
    set_input_contents(new_contents.clone());
    provide_context(CopyContents(new_contents));
  };

  let textarea_input_callback = move |event| {
    sync_input_contents(event_target_value(&event));
  };

  let formatted_json: Memo<Option<Result<String, String>>> =
    Memo::new(move |_| {
      let input_contents = input_contents.get();
      if input_contents.is_empty() {
        return None;
      }

      let formatted_json = format_json_string(&input_contents);
      Some(formatted_json)
    });

  let json_is_already_formatted = move || match formatted_json.get() {
    // formatting completed, perform comparison
    Some(Ok(v)) => Some(Some(v == input_contents.get())),
    // formatting failed
    Some(Err(_)) => Some(None),
    // no text
    None => None,
  };

  let format_button_class = move || {
    format!(
      "btn border border-gray-7 rounded {}",
      match json_is_already_formatted() {
        Some(Some(true)) => "btn-success",
        Some(Some(false)) => "btn-primary",
        Some(None) => "btn-error",
        None => "",
      }
    )
  };
  let format_button_disabled = move || match json_is_already_formatted() {
    // already formatted or no text
    Some(Some(true)) | None => true,
    _ => false,
  };
  let format_button_callback = move |_| {
    if let Some(Ok(v)) = formatted_json.get() {
      sync_input_contents(v)
    }
  };

  view! {
    <div class="relative w-full h-[calc(100%-2.5rem-2px)] p-4">
      <textarea
        class=textarea_class placeholder=placeholder
        autocapitalize="off" spellcheck="false" autofocus="true"
        on:input=textarea_input_callback
        prop:value=input_contents
      />
      <div class="absolute right-8 bottom-8 flex flex-row gap-2">
        <CopyButton />
        <button
          class=format_button_class
          disabled=format_button_disabled
          on:click=format_button_callback
        >
          "Format"
        </button>
      </div>
    </div>
  }
}

#[derive(Clone)]
pub struct CopyContents(String);

#[island]
pub fn CopyButton() -> impl IntoView {
  let click_action = {
    move |_| {
      let text: Option<CopyContents> = use_context();
      let Some(CopyContents(text)) = text else {
        return;
      };

      let clipboard = web_sys::window().unwrap().navigator().clipboard();
      let promise = clipboard.write_text(&text);
      spawn_local(async move {
        let _ =
          wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|e| {
              log!("failed to copy to clipboard: {e:?}");
            });
      });
    }
  };

  view! {
    <button class="btn border border-gray-7" on:click=click_action>
      "Copy"
    </button>
  }
}

fn format_json_string(json_string: &str) -> Result<String, String> {
  let json_value = serde_json::from_str::<serde_json::Value>(json_string)
    .map_err(|e| e.to_string())?;
  serde_json::to_string_pretty(&json_value).map_err(|e| e.to_string())
}
