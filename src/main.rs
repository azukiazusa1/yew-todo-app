use std::sync::RwLock;

use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;
use components::todo::types::Todo;

mod components;

#[function_component(App)]
fn app() -> Html {
  let todo_items = use_state(|| Vec::<Todo>::new());
  let next_id = use_state(|| 1);

  let on_add = {
    let todo_items = todo_items.clone();
    Callback::from(move |title: String| {
      let mut current_todo_items = (*todo_items).clone();
      current_todo_items.push(Todo {
        id: *next_id,
        title,
        completed: false,
      });
      next_id.set(*next_id + 1);
      todo_items.set(current_todo_items);
    })
  };

  html! {
    <>
      <Header />
      <main class="container-fluid mt-2">
        <TodoForm {on_add} />
        <TodoList todo_items={(*todo_items).clone()} />
      </main>
    </>
  }
}

fn main() {
  yew::start_app::<App>();
  wasm_logger::init(wasm_logger::Config::default());
}