use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, Result};

struct LiquidExtension;

impl zed::Extension for LiquidExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("shopify")
            .ok_or_else(|| "shopify must be installed from https://shopify.dev/docs/api/shopify-cli#installation".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["theme".to_string(), "language-server".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("liquid", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "liquid": settings
        })))
    }
}

zed::register_extension!(LiquidExtension);
