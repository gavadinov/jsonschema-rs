use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::{helpers, CompilationResult},
    validator::Validate,
};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct EnumValidator {
    options: Value,
    items: Vec<Value>,
}

impl EnumValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Array(items) = schema {
            Ok(Box::new(EnumValidator {
                options: schema.clone(),
                items: items.clone(),
            }))
        } else {
            Err(CompilationError::SchemaError)
        }
    }
}

impl Validate for EnumValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        self.items.iter().any(|item| helpers::equal(instance, item))
    }

    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if !self.is_valid(schema, instance) {
            error(ValidationError::enumeration(instance, &self.options))
        } else {
            no_error()
        }
    }
}

impl ToString for EnumValidator {
    fn to_string(&self) -> String {
        format!(
            "enum: [{}]",
            self.items
                .iter()
                .map(Value::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(EnumValidator::compile(schema))
}
