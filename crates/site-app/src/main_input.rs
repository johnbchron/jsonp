use leptos::prelude::*;
use reactive_stores::Store;

use crate::state::{MainState, MainStateStoreFields};

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
