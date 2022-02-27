use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container">
          <a class="navbar-brand" href="#">{"Yew TODO App"}</a>
        </div>
      </nav>
    }
}