# urban-system

A spanking new system for newsletter subscription

## testing

* Good old `cargo test`
* `cargo expand --test test_name`: Check if a custom port has been set, otherwise default to '5432'

## database
### initializing database locally
```bash
# starts a docker container and runs migrations
./scripts/init_db.sh

# skip running docker
SKIP_DOCKER=true ./scripts/init_db.sh
```

### creating migrations
```bash
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add migration_name
```
### running migrations
```bash
sqlx migrate run
```