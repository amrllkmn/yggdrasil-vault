mod vault;
use tonic::transport::Server;
use vault::{
    detokenize_string, tokenize_string,
    vault_server::{Vault, VaultServer},
};

#[derive(Debug, Default)]
struct VaultService; //

#[tonic::async_trait]
impl Vault for VaultService {
    // The service name defined in the protobuf file
    async fn tokenize(
        &self,
        request: tonic::Request<vault::TokenizeRequest>,
    ) -> Result<tonic::Response<vault::TokenizeResponse>, tonic::Status> {
        let req = request.into_inner();

        let resp = tokenize_string(req);
        Ok(tonic::Response::new(resp))
    }

    async fn detokenize(
        &self,
        request: tonic::Request<vault::DetokenizeRequest>,
    ) -> Result<tonic::Response<vault::DetokenizeResponse>, tonic::Status> {
        let req = request.into_inner();

        let resp = detokenize_string(req);
        Ok(tonic::Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse()?;
    let vault = VaultService::default();

    // This is to allow clients to know what the gRPC contract is for the server, so no need for sharing protobuf definitions
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(vault::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service) // add the reflection service to the transport server
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;
    Ok(())
}
