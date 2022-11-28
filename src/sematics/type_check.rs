use crate::symbol::env::Env;

use super::error::SematicsError;

pub(super) struct TypeChecker {}

impl TypeChecker {
    pub(super) fn check(env: &Env) -> Result<(), SematicsError> {
        Ok(())
    }
}
