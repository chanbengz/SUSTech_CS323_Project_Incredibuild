use thiserror::Error;

pub use SemanticError::*;

#[derive(Error, Debug)]
pub enum SemanticError {
	#[error("[Semantic Error] Type Mismatch Error[{id:?}] at line {line:?}: {message:?}")]
	TypeError{
        id: usize,
        message: String,
        line: usize,
    },
	#[error("[Semantic Error] Undefined Reference Error[{id:?}] at line {line:?}: {variable:?} undefined.")]
	ReferenceError{
        id: usize,
        variable: String,
        line: usize,
    },
	#[error("[Semantic Error] Redefinition Error[{id:?}] at line {line:?}: {variable:?} redefined.")]
	RedefinitionError{
        id: usize,
        variable: String,
        line: usize,
    },
    #[error("[Semantic Error] Invalid Operation Error[{id:?}] at line {line:?}: {message:?}")]
    ImproperUsageError{
        id: usize,
        message: String,
        line: usize,
    },
    #[error("[Semantic Error] Scope Error[{id:?}] at line {line:?}: {message:?}")]
    ScopeError{
        id: usize,
        message: String,
        line: usize,
    },
	#[error("System error: {0}")]
	SystemError(String),

}

pub type Result<T, E = SemanticError> = core::result::Result<T, E>;

pub fn map_sys_err(e: std::io::Error) -> SemanticError {
	SystemError(e.to_string())
}

pub struct SemanticErrorManager {
    cnt: usize,
    errors: Vec<SemanticError>,
}

impl SemanticErrorManager {
    pub fn new() -> Self {
        SemanticErrorManager {
            cnt: 0,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: SemanticError) {
        self.cnt += 1;
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> &Vec<SemanticError> {
        &self.errors
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }
}

