# MongoDB-Rust

Simple prototype for MongoDB CRUD in Rust.

### Environment Variables
```shell
export MONGODB_URL="mongodb+srv://<user>:<password>@<atlas>/?retryWrites=true&w=majority&appName=Cluster0"
export MONGODB_DATABASE="<database>"
export MONGODB_COLLECTION="users"
```

### Build
```
cargo build --release
```

### run
```
./user-app create "user1@foo.com"
./user-app list all
./user-app list disabled
./user-app update 63b5648cbd0aa2dad409a3d7 disabled
```

### Source
```
#Authentication
https://www.mongodb.com/docs/drivers/rust/current/fundamentals/authentication/
```