# Leptos Todo App SurrealDB (CSR)

This example creates a basic Todo app with SurrealDB, that runs in the browser (client-side rendering).

## Getting started

### Local with IndexedDB

This example is preconfigured to use IndexedDB. To get started, you'll need to have trunk installed. Check out more about it [here](https://trunkrs.dev/)

Start trunk with the following command:

```bash
trunk serve
```

You can now open the app in your browser on [http://127.0.0.1:8080/](http://127.0.0.1:8080/)

You can also set the surreal address, namespace and database with environment variables. For example:

```bash
SURREALDB_ADDRESS=indxdb://leptos_todo_app_surrealdb SURREALDB_NS=leptos_examples SURREALDB_DB=todos trunk serve
```

### Remote/Local with SurrealDB Docker image

To get started, you'll need to have Docker installed on your device first and then execute the following commands:

```bash
docker compose pull
docker compose up -d
```

This will start SurrealDB on port 8000. You can stop SurrealDB with:

```bash
docker compose stop
```

In case you prefer to install SurrealDB directly onto your system instead of using Docker, you have the option to do so. For additional information and instructions regarding the installation process, please refer to the [SurrealDB documentation](https://surrealdb.com/docs/installation).

Start trunk with the following command:

```bash
SURREALDB_ADDRESS=ws://127.0.0.1:8000 SURREALDB_USERNAME=root SURREALDB_PASSWORD=root SURREALDB_NS=leptos_examples SURREALDB_DB=todos trunk serve
```

These variables indicate the surreal address, username, password, namespace and database respectively.

## Server Side Rendering

This example cannot be built as a Server Side Rendering app. You find an SSR example here: [Leptos Todo App SurrealDB with Axum
](https://github.com/trifel/leptos_todo_app_axum_surrealdb).