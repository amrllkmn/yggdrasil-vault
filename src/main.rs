use proto::vault_server::{Vault, VaultServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("vault");
}

#[derive(Debug, Default)]
struct VaultService {}

#[tonic::async_trait]
impl Vault for VaultService {
    async fn tokenize(
        &self,
        request: tonic::Request<proto::TokenizeRequest>,
    ) -> Result<tonic::Response<proto::TokenizeResponse>, tonic::Status> {
        let req = request.into_inner();

        let resp = proto::TokenizeResponse {
            tokenized_value: req.value,
            tokenized_field: req.field,
        };

        Ok(tonic::Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let vault = VaultService::default();

    Server::builder()
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;
    println!("Listening on {addr}");
    Ok(())
}
