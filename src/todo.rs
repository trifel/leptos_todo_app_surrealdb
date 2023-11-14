use crate::{error_template::ErrorTemplate, errors::TodoAppError};
use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

const TODO_RESOURCE: &str = "todo";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: Option<Thing>,
    title: String,
    completed: bool,
}

pub async fn initialize_db() -> Result<Surreal<Any>, TodoAppError> {
    let surrealdb_address = option_env!("SURREALDB_ADDRESS")
        .unwrap_or("indxdb://leptos_todo_app_surrealdb");
    let surrealdb_username = option_env!("SURREALDB_USERNAME");
    let surrealdb_password = option_env!("SURREALDB_PASSWORD");
    let surrealdb_ns = option_env!("SURREALDB_NS").unwrap_or("leptos_examples");
    let surrealdb_db = option_env!("SURREALDB_DB").unwrap_or("todos");

    let db = connect(surrealdb_address).await.map_err(|_| {
        TodoAppError::SurrealDBError("couldn't connect to surrealDB".into())
    })?;

    if let (Some(surrealdb_username), Some(surrealdb_password)) =
        (surrealdb_username, surrealdb_password)
    {
        db.signin(Root {
            username: &surrealdb_username,
            password: &surrealdb_password,
        })
        .await
        .map_err(|_| {
            TodoAppError::SurrealDBError(
                "couldn't signin to surrealDB server".into(),
            )
        })?;
    }

    db.use_ns(surrealdb_ns)
        .use_db(surrealdb_db)
        .await
        .map_err(|_| {
            TodoAppError::SurrealDBError(
                "couldn't find db on surrealDB server".into(),
            )
        })?;

    provide_context(db.clone());

    Ok(db)
}

pub async fn db() -> Result<Surreal<Any>, TodoAppError> {
    let db = use_context::<Surreal<Any>>();
    match db {
        Some(db) => Ok(db),
        None => {
            let db = initialize_db().await?;
            Ok(db)
        }
    }
}

pub async fn get_todos() -> Result<Vec<Todo>, error::Error> {
    let db = db().await?;
    let todos = db.select(TODO_RESOURCE).await?;
    Ok(todos)
}

pub async fn add_todo(title: String) -> Result<(), error::Error> {
    let db = db().await?;

    // fake API delay
    TimeoutFuture::new(1_250).await;

    let new_todo: Result<Vec<Todo>, surrealdb::Error> = db
        .create(TODO_RESOURCE)
        .content(Todo {
            id: None,
            title,
            completed: false,
        })
        .await;

    match new_todo {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

// The struct name and path prefix arguments are optional.
pub async fn delete_todo(id: Thing) -> Result<(), error::Error> {
    let db = db().await?;
    let res: Result<Option<Todo>, surrealdb::Error> = db.delete(id).await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <header>
                <h1>"My Tasks"</h1>
            </header>
            <main>
                <Routes>
                    <Route path="" view=Todos/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo =
        create_multi_action(|title: &String| add_todo(title.clone()));
    let delete_todo = create_action(|id: &Thing| delete_todo(id.clone()));
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = create_local_resource(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    let add_todo_input_element: NodeRef<Input> = create_node_ref();

    view! {
        <div>
            <form on:submit=move |ev| {
                ev.prevent_default();
                let value = add_todo_input_element()
                    .expect("add_todo input element not found")
                    .value();
                add_todo.dispatch(value);
            }>
                <label>
                    "Add a Todo" <input node_ref=add_todo_input_element type="text" name="title"/>
                </label>
                <input type="submit" value="Add"/>
            </form>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorTemplate errors=errors/> }
                }>
                    {move || {
                        let existing_todos = {
                            move || {
                                todos
                                    .get()
                                    .map(move |todos| match todos {
                                        Err(e) => {
                                            view! { <pre class="error">"Error: " {e.to_string()}</pre> }
                                                .into_view()
                                        }
                                        Ok(todos) => {
                                            if todos.is_empty() {
                                                view! { <p>"No tasks were found."</p> }.into_view()
                                            } else {
                                                todos
                                                    .into_iter()
                                                    .map(move |todo| {
                                                        view! {
                                                            <li>
                                                                {todo.title}
                                                                <form on:submit=move |ev| {
                                                                    ev.prevent_default();
                                                                    delete_todo.dispatch(todo.id.as_ref().unwrap().clone());
                                                                }>
                                                                    <input type="submit" value="X"/>
                                                                </form>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };
                        let pending_todos = move || {
                            submissions
                                .get()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        // stop the page from reloading!

                                        <li class="pending">{move || submission.input.get()}</li>
                                    }
                                })
                                .collect_view()
                        };
                        view! {
                            // stop the page from reloading!

                            // stop the page from reloading!

                            <ul>{existing_todos} {pending_todos}</ul>
                        }
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
    }
}
