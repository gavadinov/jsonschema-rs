use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct MaxItemsValidator {
    limit: u64,
    instance_path: Vec<String>,
}

impl MaxItemsValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, instance_path: Vec<String>) -> CompilationResult {
        if let Some(limit) = schema.as_u64() {
            Ok(Box::new(MaxItemsValidator {
                limit,
                instance_path,
            }))
        } else {
            Err(CompilationError::SchemaError)
        }
    }
}

impl Validate for MaxItemsValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            if (items.len() as u64) > self.limit {
                return false;
            }
        }
        true
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            if (items.len() as u64) > self.limit {
                return error(ValidationError::max_items(
                    self.instance_path.clone(),
                    instance,
                    self.limit,
                ));
            }
        }
        no_error()
    }
}

impl ToString for MaxItemsValidator {
    fn to_string(&self) -> String {
        format!("maxItems: {}", self.limit)
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &mut CompilationContext,
) -> Option<CompilationResult> {
    Some(MaxItemsValidator::compile(
        schema,
        context.curr_instance_path.clone(),
    ))
}
