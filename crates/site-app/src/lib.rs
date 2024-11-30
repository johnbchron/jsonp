mod copy_button;
mod header;

use leptos::{config::LeptosOptions, prelude::*};
use leptos_meta::*;
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};
use reactive_stores::Store;

use self::{copy_button::CopyButton, header::Header};

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

#[derive(Clone, Store, PartialEq, Eq)]
pub enum FormattingState {
  NoText,
  SuccessfullyFormatted(String),
  FailedToFormat(String),
}

#[derive(Clone, Store, PartialEq, Eq)]
pub enum ApplyButtonState {
  NoText,
  FormattingError,
  AlreadyApplied,
  ReadyToApply,
}

#[derive(Clone, Store)]
pub struct MainState {
  input_contents: String,
}

#[derive(Clone)]
pub struct DerivedState {
  formatted_json:     Memo<FormattingState>,
  apply_button_state: Memo<ApplyButtonState>,
}

#[island]
pub fn StateProvider(children: Children) -> impl IntoView {
  let main_state = Store::new(MainState {
    input_contents: String::new(),
  });

  let formatted_json = Memo::new(move |_| {
    let input_contents = main_state.input_contents().get();
    if input_contents.is_empty() {
      return FormattingState::NoText;
    }
    match format_json_string(&input_contents) {
      Ok(v) => FormattingState::SuccessfullyFormatted(v),
      Err(e) => FormattingState::FailedToFormat(e),
    }
  });

  let apply_button_state = Memo::new(move |_| match formatted_json.get() {
    FormattingState::NoText => ApplyButtonState::NoText,
    FormattingState::FailedToFormat(_) => ApplyButtonState::FormattingError,
    FormattingState::SuccessfullyFormatted(v)
      if v == main_state.input_contents().get() =>
    {
      ApplyButtonState::AlreadyApplied
    }
    FormattingState::SuccessfullyFormatted(_) => ApplyButtonState::ReadyToApply,
  });

  let derived_state = DerivedState {
    formatted_json,
    apply_button_state,
  };

  provide_context(main_state);
  provide_context(derived_state);

  children()
}

#[island]
pub fn FormatButton() -> impl IntoView {
  let main_state = expect_context::<Store<MainState>>();
  let derived_state = expect_context::<DerivedState>();

  let format_button_base_class = "btn border border-gray-7 rounded";
  let format_button_class = move || {
    let extra_class = match derived_state.apply_button_state.get() {
      ApplyButtonState::ReadyToApply => "btn-primary",
      ApplyButtonState::AlreadyApplied => "btn-success",
      ApplyButtonState::FormattingError => "btn-error",
      _ => "",
    };
    format!("{} {}", format_button_base_class, extra_class)
  };
  let format_button_disabled = move || {
    !matches!(
      derived_state.apply_button_state.get(),
      ApplyButtonState::ReadyToApply
    )
  };

  let format_button_callback = move |_| {
    if let FormattingState::SuccessfullyFormatted(v) =
      derived_state.formatted_json.get()
    {
      main_state.input_contents().set(v);
    }
  };

  view! {
    <button
      class=format_button_class
      disabled=format_button_disabled
      on:click=format_button_callback
    >
      "Format"
    </button>
  }
}

#[island]
pub fn MainInput() -> impl IntoView {
  let textarea_class = "w-full h-full py-1 px-2 bg-backgroundSecondary \
                        text-sm text-content1 resize-none border \
                        border-gray-6 outline-none box-border font-mono \
                        relative";
  let placeholder = "Paste JSON here...";

  let main_state = expect_context::<Store<MainState>>();

  view! {
    <textarea
      class=textarea_class placeholder=placeholder autocapitalize="off"
      spellcheck="false" autofocus="true" bind:value=main_state.input_contents()
    />
  }
}

fn format_json_string(json_string: &str) -> Result<String, String> {
  let json_value = serde_json::from_str::<serde_json::Value>(json_string)
    .map_err(|e| e.to_string())?;
  serde_json::to_string_pretty(&json_value).map_err(|e| e.to_string())
}
