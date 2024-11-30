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

  let class = move || {
    format!("btn border border-gray-7 {}", match derived_state
      .apply_button_state
      .get()
    {
      ApplyButtonState::ReadyToApply => "btn-primary",
      ApplyButtonState::AlreadyApplied => "btn-success",
      ApplyButtonState::FormattingError => "btn-error",
      _ => "",
    })
  };

  let disabled = move || {
    !matches!(
      derived_state.apply_button_state.get(),
      ApplyButtonState::ReadyToApply
    )
  };

  let callback = move |_| {
    if let FormattingState::SuccessfullyFormatted(v) =
      derived_state.formatted_json.get()
    {
      main_state.input_contents().set(v);
    }
  };

  view! {
    <button class=class disabled=disabled on:click=callback>
      "Format"
    </button>
  }
}
