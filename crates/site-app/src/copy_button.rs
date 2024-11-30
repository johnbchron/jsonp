use leptos::{logging::log, prelude::*};
use reactive_stores::Store;
use wasm_bindgen_futures::spawn_local;

use crate::state::{MainState, MainStateStoreFields};

/// A button that copies the input contents to the clipboard.
#[island]
pub fn CopyButton() -> impl IntoView {
  let class = "btn border border-gray-7";

  let main_state = expect_context::<Store<MainState>>();

  let click_action =
    move |_| main_state.input_contents().with(|t| copy_to_clipboard(t));

  let disabled = move || main_state.input_contents().with(|t| t.is_empty());

  view! {
    <button class=class on:click=click_action disabled=disabled>
      "Copy"
    </button>
  }
}

fn copy_to_clipboard(text: &str) {
  let clipboard = web_sys::window().unwrap().navigator().clipboard();
  let promise = clipboard.write_text(text);
  spawn_local(async move {
    let _ = wasm_bindgen_futures::JsFuture::from(promise)
      .await
      .map_err(|e| {
        log!("failed to copy to clipboard: {e:?}");
      });
  });
}
