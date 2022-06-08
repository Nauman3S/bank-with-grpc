use tonic::{transport::Server, Request, Response, Status};

use payments::bank_server::{BankServer, Bank};
use payments::{BankPaymentRequest, BankPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BankService {}

#[tonic::async_trait]
impl Bank for BankService {
    async fn send_payment(
        &self,
        request: Request<BankPaymentRequest>,
    ) -> Result<Response<BankPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BankPaymentResponse {
            successful: true,
            message: format!("Sent ${} to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let bank_service = BankService::default();

    Server::builder()
        .add_service(BankServer::new(bank_service))
        .serve(addr)
        .await?;

    Ok(())
}
