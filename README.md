# fhe-test

## Database config

Create .env file and amend the connection string accordingly. 

```
DATABASE_URL=postgresql://postgres:postgres@localhost/lastingasset
```

Run the following command to generate Diesel configuration files:

```
diesel setup
```

This command will create a diesel.toml file in your project directory.

To create a migration:

```
diesel migration generate [migration name e.g. create_posts]
```

Diesel CLI will create two empty files for us in the required structure. We need to edit the up.sql and down.sql, then apply the new migration:

```
diesel migration run
```


## API server

Run the API server
```
cargo run --bin mobile --release
```


