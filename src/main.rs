use actix_web::{web, App, HttpRequest, HttpServer, Responder};

extern crate openidconnect;
use openidconnect::{
    AuthUrl,
    EmptyAdditionalProviderMetadata,
    IssuerUrl,
    JsonWebKeySetUrl,
    ResponseTypes,
    Scope,
    TokenUrl,
    UserInfoUrl,
};
use openidconnect::core::{
    CoreClaimName,
    CoreJwsSigningAlgorithm,
    CoreProviderMetadata,
    CoreResponseType,
    CoreSubjectIdentifierType,
};

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

async fn openid_configuration(req: HttpRequest) -> impl Responder {
    let provider_metadata = CoreProviderMetadata::new(
    IssuerUrl::new("https://id.example.com".to_string()).unwrap(),
    AuthUrl::new("https://id.example.com/authorize".to_string()).unwrap(),
    JsonWebKeySetUrl::new("https://id.example.com/jwk".to_string()).unwrap(),
    vec![
        ResponseTypes::new(vec![CoreResponseType::Code]),
        ResponseTypes::new(vec![CoreResponseType::Token, CoreResponseType::IdToken])
    ],
    vec![CoreSubjectIdentifierType::Pairwise],
    vec![CoreJwsSigningAlgorithm::RsaSsaPssSha256],
    EmptyAdditionalProviderMetadata {},
    )
    .set_token_endpoint(Some(TokenUrl::new("https://id.example.com/token".to_string()).unwrap()))
    .set_userinfo_endpoint(Some(UserInfoUrl::new("https://id.example.com/userinfo".to_string()).unwrap()))
    .set_scopes_supported(Some(vec![
        Scope::new("openid".to_string()),
        Scope::new("email".to_string()),
        Scope::new("profile".to_string()),
    ]))
    .set_claims_supported(Some(vec![
        CoreClaimName::new("sub".to_string()),
        CoreClaimName::new("aud".to_string()),
        CoreClaimName::new("email".to_string()),
        CoreClaimName::new("email_verified".to_string()),
        CoreClaimName::new("exp".to_string()),
        CoreClaimName::new("iat".to_string()),
        CoreClaimName::new("iss".to_string()),
        CoreClaimName::new("name".to_string()),
        CoreClaimName::new("given_name".to_string()),
        CoreClaimName::new("family_name".to_string()),
        CoreClaimName::new("picture".to_string()),
        CoreClaimName::new("locale".to_string()),
    ]));
    serde_json::to_string(&provider_metadata).map_err(failure::Error::from).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port: u32 = 8080;
    HttpServer::new(move || {
        App::new()
            .route("/.well-known/openid-configuration", web::get().to(openid_configuration))
    })
    .keep_alive(30)
    .client_timeout(30000)
    .workers(1)
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

