# rust-pg-docker-boilerplate

Boilerplate for Rest APIs with: 

- Rust
- Docker
- Postgres
- Tokio
- Axum
- SQLx
- Dotenvy
- Docker Compose

Before using, make sure to create a ``migrations`` folder, so SQLx can work correctly:

```bash
mkdir migrations
```

You'll also need a ``.env`` file, containing a DATABASE_URL variable, pointing to your postgres database

```bash
echo "DATABASE_URL=postgres://develop:super_strong_pass@localhost:5432/devdb" > .env
```

Now you should be able of running the application, like any other rust binary:

```bash
cargo r
```
