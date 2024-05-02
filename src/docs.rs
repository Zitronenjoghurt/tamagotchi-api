use crate::resources;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title="LemCom API",
        description="A webservice for handling LemCom online services. LemCom will be a messaging application for desktop written in Rust.\n\nAll available docs: Rapidoc (/docs), Swagger (/swagger) and Redoc (/redoc).\n\nIf you find bugs or have feedback please create an issue here: https://github.com/Zitronenjoghurt/lemcom-api/issues"
    ),
    paths(
        resources::ping::get_ping
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    modifiers(&SecurityAddon),
    components(
        schemas(),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            )
        }
    }
}
