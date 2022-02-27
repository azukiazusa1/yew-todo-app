use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
  pub title: String,
  pub completed: bool,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
  html! {
    <li class="list-group-item">
    <input class="form-check-input me-2" type="checkbox" checked={props.completed} />
      {&props.title}
    </li>
  }
}