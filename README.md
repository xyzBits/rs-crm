# rs-crm

## 构建
    - prost: 将 protobuf 编译成 Rust 代码
    - tonic: 将 Protobuf/grpc 编译成 Rust 代码
    - 使用 proto-builder-trait 

tonic: 生成两部分，client server 
tokio 下使用的数据，需要 Send Sync &'static 

```shell
cargo run --bin server
cargo run --bin clientls
```
[install sqlx ](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

```shell
# supports all databases supported by SQLx
$ cargo install sqlx-cli

# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres

# use vendored OpenSSL (build from source)
$ cargo install sqlx-cli --features openssl-vendored

# use Rustls rather than OpenSSL (be sure to add the features for the databases you intend to use!)
$ cargo install sqlx-cli --no-default-features --features rustls
```

先插入数据，再加上index，如果先加index，插入数据时会比较慢
```shell
sqlx migrate add init 
dropdb stats 
createdb stats 
sqlx migrate run 
```

docker 安装运行 [postgres](https://www.commandprompt.com/education/how-to-create-a-postgresql-database-in-docker/)

