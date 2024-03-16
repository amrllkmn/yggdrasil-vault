use proto::vault_server::{Vault, VaultServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("vault");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("vault_descriptor");
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

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;
    println!("Listening on {addr}");
    Ok(())
}
