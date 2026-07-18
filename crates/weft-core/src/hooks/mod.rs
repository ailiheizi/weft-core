//! Additive hook contracts for product packages.
//!
//! A package is a product. Another package may add behavior only by attaching
//! to a hook that product explicitly exports. There is no package resolver,
//! profile, download protocol, or package-version negotiation here.

use serde::{Deserialize, Serialize};

/// A stable hook name exported by a product package.
///
/// Breaking changes use a new name, for example
/// `weft_claw.turn.before_tools.v2`; no separate version negotiation is needed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookContract {
    pub id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookPhase {
    Before,
    Around,
    After,
}

/// One package's declared additive change to a product hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookAttachment {
    pub package: String,
    pub hook: String,
    pub phase: HookPhase,
    pub entry: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookValidationError {
    UnknownHook(String),
    DuplicateAttachment { package: String, hook: String },
}

impl std::fmt::Display for HookValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHook(hook) => write!(formatter, "package targets unknown hook '{hook}'"),
            Self::DuplicateAttachment { package, hook } => {
                write!(formatter, "package '{package}' attaches more than once to hook '{hook}'")
            }
        }
    }
}

impl std::error::Error for HookValidationError {}

/// Validates attachments and returns a deterministic dispatch order.
///
/// The package name provides the tie breaker, so hook manifests do not need a
/// priority language or a dependency/order solver.
pub fn validate_and_order(
    contracts: &[HookContract],
    attachments: &[HookAttachment],
) -> Result<Vec<HookAttachment>, HookValidationError> {
    let mut result = attachments.to_vec();

    for attachment in &result {
        if !contracts.iter().any(|contract| contract.id == attachment.hook) {
            return Err(HookValidationError::UnknownHook(attachment.hook.clone()));
        }
    }

    result.sort_by(|left, right| {
        (&left.hook, left.phase, &left.package).cmp(&(&right.hook, right.phase, &right.package))
    });

    for pair in result.windows(2) {
        if pair[0].hook == pair[1].hook && pair[0].package == pair[1].package {
            return Err(HookValidationError::DuplicateAttachment {
                package: pair[0].package.clone(),
                hook: pair[0].hook.clone(),
            });
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
        }
    }

    fn attachment(package: &str) -> HookAttachment {
        HookAttachment {
            package: package.into(),
            hook: "weft_claw.turn.before_tools.v1".into(),
            phase: HookPhase::Before,
            entry: "package.wasm".into(),
        }
    }

    #[test]
    fn additive_packages_have_a_deterministic_order_without_a_solver() {
        let ordered = validate_and_order(&[contract()], &[attachment("z-tools"), attachment("a-tools")])
            .expect("valid attachments");
        assert_eq!(ordered[0].package, "a-tools");
        assert_eq!(ordered[1].package, "z-tools");
    }

    #[test]
    fn a_package_cannot_attach_to_a_hook_the_product_did_not_export() {
        let mut attachment = attachment("custom-tools");
        attachment.hook = "weft_claw.turn.hidden.v1".into();
        assert!(matches!(
            validate_and_order(&[contract()], &[attachment]),
            Err(HookValidationError::UnknownHook(_))
        ));
    }
}
