# Leptos Todo App SurrealDB (CSR)

This example creates a basic todo app with SurrealDB with Client Side Rendering. 

## Getting started with SurrealDB

To get started, you'll need to have Docker installed on your device first and then execute the following commands:

```bash
docker compose pull
docker compose up -d
```

This will start SurrealDB on port 8000. You can stop SurrealDB with: 
```bash
docker compose stop
```

In case you prefer to install SurrealDB directly onto your system instead of using Docker, you have the option to do so. For additional information and instructions regarding the installation process, please refer to the [SurrealDB documentation](https://surrealdb.com/docs/installation). After the installation, you may need to set several parameters as environment variables, e.g.:

```bash
SURREALDB_SERVER=127.0.0.1 SURREALDB_PORT=8000 SURREALDB_USERNAME=root SURREALDB_PASSWORD=root SURREALDB_NS=leptos_examples SURREALDB_DB=todos trunk serve
```
These variables indicate the server address, port, username, password, namespace, and database respectively. If you are using this example with Docker, you don't need to set these variables as they are already set in the `docker-compose.yml` file.

## Client Side Rendering
Before you start, you'll need to have trunk installed. Check out more about it [here](https://trunkrs.dev/)

```bash
cargo install --locked trunk
``` 
2. Build the site in watch mode, recompiling on file changes
```bash
trunk serve
```

Open browser on [http://localhost:8080/](http://localhost:8080/)

## Server Side Rendering

This example cannot be built as a Server Side Rendering app. You find an SSR example here: [Leptos Todo App SurrealDB with Axum
](https://github.com/trifel/leptos_todo_app_axum_surrealdb).