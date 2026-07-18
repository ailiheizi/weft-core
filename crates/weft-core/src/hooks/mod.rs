//! Stable, additive extension points for Weft modules.
//!
//! Hooks are contracts, not dependencies. A base module explicitly exports a
//! hook and an overlay explicitly attaches to it. The runtime will eventually
//! dispatch handlers through the package bridge; this module owns the shared
//! validation and deterministic ordering rules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookContract {
    /// Stable, versioned id, for example `weft_claw.turn.before_tools.v1`.
    pub id: String,
    /// Version of the hook payload contract.
    pub api_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookPhase {
    Before,
    Around,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookAttachment {
    pub overlay: String,
    pub hook: String,
    pub api_version: u32,
    pub phase: HookPhase,
    #[serde(default)]
    pub order: i32,
    pub entry: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookValidationError {
    UnknownHook(String),
    IncompatibleApiVersion {
        hook: String,
        expected: u32,
        received: u32,
    },
    DuplicateOverlay(String),
}

impl std::fmt::Display for HookValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHook(hook) => write!(formatter, "overlay targets unknown hook '{hook}'"),
            Self::IncompatibleApiVersion {
                hook,
                expected,
                received,
            } => write!(
                formatter,
                "overlay targets hook '{hook}' API v{received}, but the base module exports v{expected}"
            ),
            Self::DuplicateOverlay(overlay) => {
                write!(formatter, "overlay '{overlay}' attaches more than once to the same hook phase")
            }
        }
    }
}

impl std::error::Error for HookValidationError {}

/// Validates explicit overlay attachments and returns deterministic dispatch
/// order. `replace` is deliberately absent: v2 extensions are additive only.
pub fn validate_and_order(
    contracts: &[HookContract],
    attachments: &[HookAttachment],
) -> Result<Vec<HookAttachment>, HookValidationError> {
    let mut result = attachments.to_vec();

    for attachment in &result {
        let Some(contract) = contracts.iter().find(|contract| contract.id == attachment.hook)
        else {
            return Err(HookValidationError::UnknownHook(attachment.hook.clone()));
        };
        if contract.api_version != attachment.api_version {
            return Err(HookValidationError::IncompatibleApiVersion {
                hook: attachment.hook.clone(),
                expected: contract.api_version,
                received: attachment.api_version,
            });
        }
    }

    result.sort_by(|left, right| {
        (&left.hook, left.phase, left.order, &left.overlay).cmp(&(
            &right.hook,
            right.phase,
            right.order,
            &right.overlay,
        ))
    });

    for pair in result.windows(2) {
        if pair[0].hook == pair[1].hook
            && pair[0].phase == pair[1].phase
            && pair[0].overlay == pair[1].overlay
        {
            return Err(HookValidationError::DuplicateOverlay(pair[0].overlay.clone()));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{validate_and_order, HookAttachment, HookContract, HookPhase, HookValidationError};

    fn contract() -> HookContract {
        HookContract {
            id: "weft_claw.turn.before_tools.v1".into(),
            api_version: 1,
        }
    }

    fn attachment(overlay: &str, order: i32) -> HookAttachment {
        HookAttachment {
            overlay: overlay.into(),
            hook: "weft_claw.turn.before_tools.v1".into(),
            api_version: 1,
            phase: HookPhase::Before,
            order,
            entry: "overlay.wasm".into(),
        }
    }

    #[test]
    fn attachments_are_ordered_without_dependency_resolution() {
        let ordered = validate_and_order(&[contract()], &[attachment("later", 10), attachment("first", 0)])
            .expect("valid attachments");
        assert_eq!(ordered[0].overlay, "first");
        assert_eq!(ordered[1].overlay, "later");
    }

    #[test]
    fn incompatible_hook_contract_is_rejected() {
        let mut attachment = attachment("old-overlay", 0);
        attachment.api_version = 2;
        assert!(matches!(
            validate_and_order(&[contract()], &[attachment]),
            Err(HookValidationError::IncompatibleApiVersion { .. })
        ));
    }
}
