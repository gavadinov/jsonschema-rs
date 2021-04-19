use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct RequiredValidator {
    required: Vec<String>,
    instance_path: Vec<String>,
}

impl RequiredValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, instance_path: Vec<String>) -> CompilationResult {
        match schema {
            Value::Array(items) => {
                let mut required = Vec::with_capacity(items.len());
                for item in items {
                    match item {
                        Value::String(string) => required.push(string.clone()),
                        _ => return Err(CompilationError::SchemaError),
                    }
                }
                Ok(Box::new(RequiredValidator {
                    required,
                    instance_path,
                }))
            }
            _ => Err(CompilationError::SchemaError),
        }
    }
}

impl Validate for RequiredValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            self.required
                .iter()
                .all(|property_name| item.contains_key(property_name))
        } else {
            true
        }
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Object(item) = instance {
            for property_name in &self.required {
                if !item.contains_key(property_name) {
                    return error(ValidationError::required(
                        self.instance_path.clone(),
                        instance,
                        property_name.clone(),
                    ));
                }
            }
        }
        no_error()
    }
}

impl ToString for RequiredValidator {
    fn to_string(&self) -> String {
        format!("required: [{}]", self.required.join(", "))
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &mut CompilationContext,
) -> Option<CompilationResult> {
    Some(RequiredValidator::compile(
        schema,
        context.curr_instance_path.clone(),
    ))
}
