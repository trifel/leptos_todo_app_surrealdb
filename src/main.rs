use crate::todo::*;
use leptos::*;
use leptos_todo_app_surrealdb::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Error);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <TodoApp/> })
}
