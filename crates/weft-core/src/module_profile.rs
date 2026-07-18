//! Deterministic module profiles used by package system v2.
//!
//! A profile is an explicit desired state. It lists exact artifacts and their
//! overlays, so activating an update never invokes a transitive dependency
//! solver or merges registry sources.

use crate::hooks::{validate_and_order, HookAttachment, HookContract, HookValidationError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleArtifact {
    pub id: String,
    pub version: String,
    pub url: String,
    pub sha256: String,
    pub core_compatibility: String,
    #[serde(default)]
    pub hooks: Vec<HookContract>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleOverlay {
    pub id: String,
    pub version: String,
    pub target: String,
    pub artifact: ModuleArtifact,
    pub attachment: HookAttachment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleProfile {
    pub schema_version: u32,
    pub name: String,
    pub modules: Vec<ModuleArtifact>,
    #[serde(default)]
    pub overlays: Vec<ModuleOverlay>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileValidationError {
    DuplicateModule(String),
    MissingOverlayTarget { overlay: String, target: String },
    Hook(HookValidationError),
}

impl std::fmt::Display for ProfileValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateModule(id) => write!(formatter, "profile declares module '{id}' more than once"),
            Self::MissingOverlayTarget { overlay, target } => {
                write!(formatter, "overlay '{overlay}' targets module '{target}', which is not in the profile")
            }
            Self::Hook(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ProfileValidationError {}

impl From<HookValidationError> for ProfileValidationError {
    fn from(error: HookValidationError) -> Self {
        Self::Hook(error)
    }
}

impl ModuleProfile {
    /// Validates explicit composition and returns the handler dispatch order.
    pub fn validate(&self) -> Result<Vec<HookAttachment>, ProfileValidationError> {
        let mut module_ids = HashSet::new();
        for module in &self.modules {
            if !module_ids.insert(module.id.as_str()) {
                return Err(ProfileValidationError::DuplicateModule(module.id.clone()));
            }
        }

        let mut contracts = Vec::new();
        let mut attachments = Vec::new();
        for overlay in &self.overlays {
            let Some(target) = self.modules.iter().find(|module| module.id == overlay.target) else {
                return Err(ProfileValidationError::MissingOverlayTarget {
                    overlay: overlay.id.clone(),
                    target: overlay.target.clone(),
                });
            };
            contracts.extend(target.hooks.clone());
            attachments.push(overlay.attachment.clone());
        }

        validate_and_order(&contracts, &attachments).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::{ModuleArtifact, ModuleOverlay, ModuleProfile, ProfileValidationError};
    use crate::hooks::{HookAttachment, HookContract, HookPhase};

    fn base() -> ModuleArtifact {
        ModuleArtifact {
            id: "weft-claw".into(),
            version: "0.3.0".into(),
            url: "https://example.invalid/weft-claw.tar.gz".into(),
            sha256: "abc".into(),
            core_compatibility: ">=0.2,<0.3".into(),
            hooks: vec![HookContract {
                id: "weft_claw.turn.before_tools.v1".into(),
                api_version: 1,
            }],
        }
    }

    #[test]
    fn profile_requires_an_explicit_overlay_target() {
        let overlay_artifact = base();
        let profile = ModuleProfile {
            schema_version: 1,
            name: "default".into(),
            modules: vec![],
            overlays: vec![ModuleOverlay {
                id: "custom-tools".into(),
                version: "1.0.0".into(),
                target: "weft-claw".into(),
                artifact: overlay_artifact,
                attachment: HookAttachment {
                    overlay: "custom-tools".into(),
                    hook: "weft_claw.turn.before_tools.v1".into(),
                    api_version: 1,
                    phase: HookPhase::Before,
                    order: 0,
                    entry: "overlay.wasm".into(),
                },
            }],
        };
        assert!(matches!(
            profile.validate(),
            Err(ProfileValidationError::MissingOverlayTarget { .. })
        ));
    }

    #[test]
    fn profile_accepts_a_base_module_and_matching_overlay() {
        let base_module = base();
        let profile = ModuleProfile {
            schema_version: 1,
            name: "default".into(),
            modules: vec![base_module],
            overlays: vec![ModuleOverlay {
                id: "custom-tools".into(),
                version: "1.0.0".into(),
                target: "weft-claw".into(),
                artifact: base(),
                attachment: HookAttachment {
                    overlay: "custom-tools".into(),
                    hook: "weft_claw.turn.before_tools.v1".into(),
                    api_version: 1,
                    phase: HookPhase::Before,
                    order: 0,
                    entry: "overlay.wasm".into(),
                },
            }],
        };
        assert_eq!(profile.validate().expect("valid profile").len(), 1);
    }
}
