use proto::vault_server::{Vault, VaultServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("vault"); // import the generated grpc package from running build.rs

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("vault_descriptor"); // import the file descriptor for reflection
}

#[derive(Debug, Default)]
struct VaultService {} //

#[tonic::async_trait]
impl Vault for VaultService {
    // The service name defined in the protobuf file
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

    async fn detokenize(
        &self,
        request: tonic::Request<proto::DetokenizeRequest>,
    ) -> Result<tonic::Response<proto::DetokenizeResponse>, tonic::Status> {
        let req = request.into_inner();

        let resp = proto::DetokenizeResponse {
            detokenized_value: req.value,
            detokenized_field: req.field,
        };

        Ok(tonic::Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse()?;
    let vault = VaultService::default();

    // This is to allow clients to know what the gRPC contract is for the server, so no need for sharing protobuf definitions
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service) // add the reflection service to the transport server
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;
    println!("Listening on {addr}");
    Ok(())
}
