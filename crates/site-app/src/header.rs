use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
  view! {
    <div class="h-10 bg-backgroundSecondary px-4 flex items-center border-b border-gray-7">
      <p class="text-lg">"JSON Prettifier"</p>
      <div class="flex-grow"/>
      <a
        class="text-sm text-content2 underline hover:text-content1 transition-colors"
        href="https://github.com/johnbchron/jsonp"
        target="_blank" rel="noopener noreferrer"
      >
        "v"
        { version::version!() }
      </a>
    </div>
  }
}
