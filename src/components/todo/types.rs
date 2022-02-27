#[derive(PartialEq, Clone)]
pub struct Todo {
  pub id: usize,
  pub title: String,
  pub completed: bool,
}