use thiserror::Error;

#[derive(Clone, Error, Debug)]
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
    #[error("[Semantic Error] Not Implemented Feature Error at line {line:?}: {message:?}")]
    NotImplementedFeatureError{
        message: String,
        line: usize
    },
	#[error("System error: {0}")]
	SystemError(String),

}

pub struct SemanticErrorManager {
    cnt: usize,
    errors: Vec<SemanticError>,
    line: usize,
}

impl SemanticErrorManager {
    pub fn new() -> Self {
        SemanticErrorManager {
            cnt: 0,
            errors: Vec::new(),
            line: 0,
        }
    }

    pub fn add_error(&mut self, mut error: SemanticError) {
        self.cnt += 1;
        error.update_line(self.line);
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> &Vec<SemanticError> {
        &self.errors
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn update_line(&mut self) {
        self.line += 1;
    }

    pub fn update_line_with_value(&mut self, value: usize) {
        self.line = value;
    }
}

impl SemanticError {
    fn update_line(&mut self, line: usize) {
        match self {
            SemanticError::TypeError{line: ref mut l, ..} => *l = line,
            SemanticError::ReferenceError{line: ref mut l, ..} => *l = line,
            SemanticError::RedefinitionError{line: ref mut l, ..} => *l = line,
            SemanticError::ImproperUsageError{line: ref mut l, ..} => *l = line,
            SemanticError::ScopeError{line: ref mut l, ..} => *l = line,
            SemanticError::NotImplementedFeatureError{line: ref mut l, ..} => *l = line,
            _ => {}
        }
    }
}