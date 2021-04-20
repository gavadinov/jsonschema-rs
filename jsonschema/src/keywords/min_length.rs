use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct MinLengthValidator {
    limit: u64,
    instance_path: Vec<String>,
}

impl MinLengthValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, instance_path: Vec<String>) -> CompilationResult {
        if let Some(limit) = schema.as_u64() {
            Ok(Box::new(MinLengthValidator {
                limit,
                instance_path,
            }))
        } else {
            Err(CompilationError::SchemaError)
        }
    }
}

impl Validate for MinLengthValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::String(item) = instance {
            if (item.chars().count() as u64) < self.limit {
                return false;
            }
        }
        true
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::String(item) = instance {
            if (item.chars().count() as u64) < self.limit {
                return error(ValidationError::min_length(
                    self.instance_path.clone(),
                    instance,
                    self.limit,
                ));
            }
        }
        no_error()
    }
}

impl ToString for MinLengthValidator {
    fn to_string(&self) -> String {
        format!("minLength: {}", self.limit)
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &mut CompilationContext,
) -> Option<CompilationResult> {
    Some(MinLengthValidator::compile(
        schema,
        context.curr_instance_path.clone(),
    ))
}
