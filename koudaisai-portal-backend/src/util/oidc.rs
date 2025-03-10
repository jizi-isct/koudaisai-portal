use oauth2::basic::{BasicErrorResponseType, BasicRevocationErrorResponse};
use oauth2::StandardRevocableToken;
use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreGenderClaim, CoreJsonWebKey,
    CoreJweContentEncryptionAlgorithm, CoreTokenIntrospectionResponse, CoreTokenResponse,
};
use openidconnect::{
    Client, EmptyAdditionalClaims, EndpointMaybeSet, EndpointNotSet, EndpointSet,
    StandardErrorResponse,
};

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
