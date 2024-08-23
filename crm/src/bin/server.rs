use anyhow::Result;
use crm::pb::user_service_server::{UserService, UserServiceServer};
use crm::pb::{CreateUserRequest, GetUserRequest, User};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};

#[derive(Default)]
pub struct UserServer;

// 这个 UserService 是定义在 protobuf 中的 UserService
// 经过rust 编译后，生成了一个 trait
// 以前 async 不能在trait 中使用，所以需要加这个注解，才可以在实现中添加 async 函数
#[async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("get_user: {:?}", input);
        Ok(Response::new(User::default()))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        let user = User::new(1, &input.name, &input.email);

        Ok(Response::new(user))
    }
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     let addr: SocketAddr = "[::1]:50051".parse()?;
//     let user_server = UserServer::default();
//
//     println!("UserService listening on {:?}", addr);
//
//     Server::builder()
//         .add_service(UserServiceServer::new(user_server))
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<()> {

    tokio::spawn(async move {
        let addr: SocketAddr = "[::1]:50051".parse().unwrap();
        let user_server = UserServer::default();

        println!("UserService listening on {:?}", addr);

        Server::builder()
            .add_service(UserServiceServer::new(user_server))
            .serve(addr)
            .await
            .unwrap();
    });

    // 如果这里不 sleep 主程序就直接结束了
    tokio::time::sleep(Duration::from_secs(100 * 1000)).await;


    Ok(())
}
