use leptos::{logging::log, prelude::*};
use reactive_stores::Store;
use wasm_bindgen_futures::spawn_local;

use crate::{MainState, MainStateStoreFields};

#[island]
pub fn CopyButton() -> impl IntoView {
  let click_action = {
    move |_| {
      let main_state = expect_context::<Store<MainState>>();
      let text = main_state.input_contents().get();

      if text.is_empty() {
        return;
      }

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
