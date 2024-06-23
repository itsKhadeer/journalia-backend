# journalia-backend

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy):

    ```bash
    rustup component add rustfmt clippy
    ```

3. Clone and change directory into this repo
4. Install pre-commit hooks
    - Install Python and pip
    - Install pre-commit package:

    ```bash
    pip install pre-commit
    pre-commit install
    ```

5. Run:

    ```bash
    cp .env.example .env
    ```

    and fill the env variables

<!-- markdownlint-disable MD029 -->

### With Docker

6. Install [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/)
7. Start the services:

    ```bash
    docker-compose up
    ```

8. To generate migrations or run any command on the rust container, prefix it with `docker-compose exec server` like so:

    ```bash
    docker-compose exec server diesel migration generate <migration-name>
    ```

### Bare metal

6. Install [PostgreSQL](https://www.postgresql.org/download/)
7. Install diesel_cli:

    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```

8. Install [cargo-watch](https://github.com/watchexec/cargo-watch) for hot reload (optional):

    ```bash
    cargo install cargo-watch
    ```

9. To run migrations,

    ```bash
    diesel migration run
    ```

10. Run:

    ```bash
    cargo watch -i logs -x run
    ```

    to start the server with hot reload, otherwise

    ```bash
    cargo run
    ```

Seed the dummy data using:

```bash
psql -U journalia --set ON_ERROR_STOP=on < dummy_data.sql
```

For docker, prefix the above command with ```docker-compose exec -T db```

## Deployment

1. Run:

    ```bash
    cp .env.example .env
    ```

    and fill the env variables


2. Start the services:

    ```bash
    ./prod-run.sh up
    ```

	and build the images again with
	```bash
	./prod-run.sh up build
	```

3. Down the services:

	```bash
	./prod-run.sh down
	```

4. Logs for the services:

	```bash
	./prod-run.sh logs
	```

You can attach to containers with:

```bash
docker attach <container-name>
```
