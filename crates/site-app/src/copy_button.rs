use leptos::{logging::log, prelude::*};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct CopyContents(String);

impl CopyContents {
  pub fn new(text: String) -> Self { Self(text) }
}

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
