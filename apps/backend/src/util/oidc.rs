use crate::config::KeyCloak;
use oauth2::StandardRevocableToken;
use oauth2::basic::{BasicErrorResponseType, BasicRevocationErrorResponse};
use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreGenderClaim, CoreJsonWebKey,
    CoreJweContentEncryptionAlgorithm, CoreProviderMetadata, CoreTokenIntrospectionResponse,
    CoreTokenResponse,
};
use openidconnect::{
    Client, ClientId, ClientSecret, EmptyAdditionalClaims, EndpointMaybeSet, EndpointNotSet,
    EndpointSet, IssuerUrl, RedirectUrl, StandardErrorResponse,
};
use tracing::instrument;

pub type OIDCClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<BasicErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

/// Keycloak 設定からプロバイダメタデータを discovery して OIDC クライアントを構築する。
#[instrument(skip(keycloak))]
pub async fn from_config(keycloak: &KeyCloak, redirect_url: String) -> OIDCClient {
    let http_client = reqwest::Client::new();

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(keycloak.issuer.clone()).unwrap(),
        &http_client,
    )
    .await
    .unwrap();

    CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(keycloak.id.clone()),
        Some(ClientSecret::new(keycloak.secret.clone())),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
