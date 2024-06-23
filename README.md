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

Seed the dummy data using:

```bash
docker-compose exec -T db psql -U journalia --set ON_ERROR_STOP=on < dummy_data.sql
```


