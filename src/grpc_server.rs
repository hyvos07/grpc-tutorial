use services::{
    PaymentRequest, PaymentResponse,
    payment_service_server::{PaymentService, PaymentServiceServer},
};
use tonic::{Request, Response, Status, transport::Server};

pub mod services {
    tonic::include_proto!("services");
}

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);
        Ok(Response::new(PaymentResponse { success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse();
    let payment_service = MyPaymentService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr?)
        .await?;
    Ok(())
}
