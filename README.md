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