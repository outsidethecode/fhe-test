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



Run the API server
```
cargo run --bin mobile --release
```


