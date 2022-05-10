use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::{metadata::MetadataValue, Request};

pub mod auth {
    tonic::include_proto!("anyon.authmanager.v1.gateway");
}

use auth::gateway_client::GatewayClient;
use auth::LoginRequest;

pub mod pbsnow {
    tonic::include_proto!("anyon.public.snowflake");
}

use pbsnow::circuit_api_client::CircuitApiClient;
use pbsnow::Circuit;
use pbsnow::instruction;
use pbsnow::Instruction;
use pbsnow::SubmitJobRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pem = tokio::fs::read("certs/user-gateway.crt").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("user-gateway");

    let channel = Channel::from_static("http://user-gateway:8080")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut auth_client = GatewayClient::new(channel);

    let request = tonic::Request::new(LoginRequest{
        username: "alice".to_string(),
        password: "password".to_string(),
    });

    let response = auth_client.login(request).await?;

    println!("RESPONSE={:?}", response.get_ref().access_token);

    // ---------------

    let pem = tokio::fs::read("certs/user-gateway.crt").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("user-gateway");

    let channel = Channel::from_static("http://user-gateway:8080")
        .tls_config(tls)?
        .connect()
        .await?;

    // let mut snowflake_client = CircuitApiClient::new(channel);
    let t = format!("Bearer {}", response.get_ref().access_token);
    println!("{}", t);
    let token: MetadataValue<_> = t.parse()?;
    let mut snowflake_client = CircuitApiClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let v1 = instruction::Parameter{
        name: "a".to_string(),
        value: 1.0,
    };

    let i1 = Instruction{
        symbol: "iswap".to_string(),
        parameters: [v1].to_vec(),
        qubits: [1, 2, 3].to_vec(),
        classical_bits: [1, 2, 3].to_vec(),
    };

    let request = tonic::Request::new(SubmitJobRequest{
        circuit: Some(Circuit{
            instructions: [i1].to_vec(),
        }),
        shots_count: 1,
        owner: "alice".to_string(),
        token: "fake".to_string(),
    });

    let response = snowflake_client.submit_job(request).await?;

    println!("SNOWFLAKE RESPONSE={:?}", response);

    Ok(())
}

