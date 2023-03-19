# Rust To-Do List

## Dev Test

```sh
# Test database
cargo watch -q -c -w src/ -x 'test model_db -- --test-threads=1 --nocapture'

# Test Todo model
cargo watch -q -c -w src/ -x 'test model_todo -- --test-threads=1 --nocapture' 
```

## Dev Web

```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```

## Database

```sh
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal)
docker exec -it -u postgres pg psql
```
