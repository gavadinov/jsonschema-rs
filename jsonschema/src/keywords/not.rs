use crate::{
    compilation::{compile_validators, context::CompilationContext, JSONSchema},
    error::{error, no_error, ErrorIterator, ValidationError},
    keywords::{format_validators, CompilationResult, Validators},
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct NotValidator {
    // needed only for error representation
    original: Value,
    validators: Validators,
    instance_path: Vec<String>,
}

impl NotValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, context: &CompilationContext) -> CompilationResult {
        Ok(Box::new(NotValidator {
            original: schema.clone(),
            validators: compile_validators(schema, context)?,
            instance_path: context.curr_instance_path.clone(),
        }))
    }
}

impl Validate for NotValidator {
    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        !self
            .validators
            .iter()
            .all(|validator| validator.is_valid(schema, instance))
    }

    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::not(
                self.instance_path.clone(),
                instance,
                self.original.clone(),
            ))
        }
    }
}

impl ToString for NotValidator {
    fn to_string(&self) -> String {
        format!("not: {}", format_validators(&self.validators))
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &mut CompilationContext,
) -> Option<CompilationResult> {
    Some(NotValidator::compile(schema, context))
}
