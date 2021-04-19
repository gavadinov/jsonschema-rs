use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct MaxPropertiesValidator {
    limit: u64,
    instance_path: Vec<String>,
}

impl MaxPropertiesValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, instance_path: Vec<String>) -> CompilationResult {
        if let Some(limit) = schema.as_u64() {
            Ok(Box::new(MaxPropertiesValidator {
                limit,
                instance_path,
            }))
        } else {
            Err(CompilationError::SchemaError)
        }
    }
}

impl Validate for MaxPropertiesValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            if (item.len() as u64) > self.limit {
                return false;
            }
        }
        true
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Object(item) = instance {
            if (item.len() as u64) > self.limit {
                return error(ValidationError::max_properties(
                    self.instance_path.clone(),
                    instance,
                    self.limit,
                ));
            }
        }
        no_error()
    }
}

impl ToString for MaxPropertiesValidator {
    fn to_string(&self) -> String {
        format!("maxProperties: {}", self.limit)
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &mut CompilationContext,
) -> Option<CompilationResult> {
    Some(MaxPropertiesValidator::compile(
        schema,
        context.curr_instance_path.clone(),
    ))
}
