// use tonic::{transport::Server, Request, Response, Status};
//
// use auth::gateway_server::{Gateway, GatewayServer};
// use auth::{LoginRequest, LoginResponse};
//
// pub mod auth {
//     tonic::include_proto!("anyon.authmanager.v1.gateway");
// }
//
// #[derive(Debug, Default)]
// pub struct MyGateway {}
//
// #[tonic::async_trait]
// impl Gateway for MyGateway {
//     async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
//         println!("Got a request: {:?}", request);
//
//         let reply = auth::LoginResponse {
//             access_token: "access-token".to_string(),
//         };
//
//         Ok(Response::new(reply))
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "127.0.0.1:50055".parse()?;
//     let greeter = MyGateway::default();
//
//     Server::builder()
//         .add_service(GatewayServer::new(greeter))
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }
