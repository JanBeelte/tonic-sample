use proto::calc_server::{Calc, CalcServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calc");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calc_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

impl CalculatorService {}

#[tonic::async_trait]
impl Calc for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalcReq>,
    ) -> Result<tonic::Response<proto::CalcResp>, tonic::Status> {
        let input = request.get_ref();

        let response = proto::CalcResp {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalcReq>,
    ) -> Result<tonic::Response<proto::CalcResp>, tonic::Status> {
        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument(
                "division by zero not permitted",
            ));
        }

        let response = proto::CalcResp {
            result: input.a / input.b,
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    let calc = CalculatorService {};
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(tonic_web::enable(CalcServer::new(calc)))
        .serve(addr)
        .await?;

    Ok(())
}
