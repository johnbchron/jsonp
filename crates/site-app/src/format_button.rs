use leptos::prelude::*;
use reactive_stores::Store;

use crate::state::{
  ApplyButtonState, DerivedState, FormattingState, MainState,
  MainStateStoreFields,
};

/// A button that formats the input JSON.
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
