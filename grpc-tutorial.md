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