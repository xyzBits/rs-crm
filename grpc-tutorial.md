# Rust GRPC 教程

## 1. 编译 protobuf 文件
```protobuf
package crm;

import "google/protobuf/timestamp.proto";

message User {
    uint64 id = 1;
    string name = 2;
    string email = 3;
    google.protobuf.Timestamp created_at = 4;
}

message GetUserRequest {
    uint64 id = 1;
}

message CreateUserRequest {
    string name = 1;
    string email = 2;
}

service UserService {
    rpc GetUser(GetUserRequest) returns (User) {}
    rpc CreateUser(CreateUserRequest) returns (User) {}
}
```

编译脚本
```rust
use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    // 默认情况下，build.rs 得到的中间文件保存在 out_dir 环境指定的目录中
    // 如果没有明确设置 out_dir 环境变量，则默认为 cargo 的构建目录，target
    //
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();

    builder
        .out_dir("src/pb") // 输出的路径，此处指定为项目 src/pb
        .with_serde(
            &["User"],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        // 指定要编译的 proto 文件路径列表，第二个参数是提供protobuf的扩展路径
        // 因为 protobuf 官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .compile(
            &[
                "../protos/metadata/messages.proto",
                "../protos/metadata/rpc.proto",
            ],
            // 这个参数非常重要，如果上面的两个proto文件之间相互有依赖，那么这个参数的目录一定得是他们所在目录的父目录
            // 因为要去这个父目录中找依赖的 proto文件
            &["../protos/"],
        )?; // import "user-stats/messages.proto";
            // &["../protos/user-stats"])?;// import "messages.proto";

    Ok(())
}
```

## 2. 对于 `UserService` 会编译成一个 trait 
```rust
#[async_trait]
    pub trait UserService: Send + Sync + 'static {
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status>;
        async fn create_user(
            &self,
            request: tonic::Request<super::CreateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status>;
    }
```

## 3. 然后 server 端定义一个 struct 实现这个 trait
```rust

#[derive(Default)]
pub struct UserServer;

// 这个 UserService 是定义在 protobuf 中的 UserService
// 经过rust 编译后，生成了一个 trait 
#[async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        todo!()
    }
}
```

## 4. 启动服务端
```rust

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let user_server = UserServer::default();

    println!("UserService listening on {:?}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_server))
        .serve(addr)
        .await?;

    Ok(())
}
```

## 5. 启动客户端
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "Tom".to_string(),
        email: "tom@gmail.com".to_string(),
    });

    let response = client.create_user(request).await?;
    println!("RESPONSE = {:?}", response);

    Ok(())
}
```