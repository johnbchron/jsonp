use leptos::prelude::*;
use reactive_stores::Store;

use crate::format_json_string;

/// The state of the JSON formatting process.
#[derive(Clone, Store, PartialEq, Eq)]
pub enum FormattingState {
  NoText,
  SuccessfullyFormatted(String),
  FailedToFormat(String),
}

/// The state of the apply button.
#[derive(Clone, Store, PartialEq, Eq)]
pub enum ApplyButtonState {
  NoText,
  FormattingError,
  AlreadyApplied,
  ReadyToApply,
}

/// The main state of the app. These items are set directly by user input.
#[derive(Clone, Store)]
pub struct MainState {
  pub input_contents: String,
}

/// The derived state of the app. These items are calculated from the main
/// state.
#[derive(Clone)]
pub struct DerivedState {
  pub formatted_json:     Memo<FormattingState>,
  pub apply_button_state: Memo<ApplyButtonState>,
}

/// The state provider for the app. Provides the main and derived states by
/// context.
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
