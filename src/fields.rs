use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tantivy::schema::FieldType;
use tantivy::Index;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Field {
    name: String,
    r#type: String,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Field '{}' ({})", self.name, self.r#type)
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Fields(Vec<Field>);

impl Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Available fields")?;

        for fi in &self.0 {
            write!(f, "{}", fi)?;
        }

        Ok(())
    }
}

impl From<&Index> for Fields {
    fn from(index: &Index) -> Self {
        let schema = index.schema();
        let mut fields: Vec<Field> = Vec::with_capacity(schema.fields().count());

        for (_, field_entry) in schema.fields() {
            let data_type = match field_entry.field_type() {
                FieldType::Str(_) => "text",
                FieldType::U64(_) => "u64",
                FieldType::I64(_) => "i64",
                FieldType::F64(_) => "f64",
                FieldType::Date(_) => "date",
                FieldType::Facet(_) => "facet",
                FieldType::Bytes(_) => "bytes",
                FieldType::JsonObject(_) => "JSON object",
            };
            fields.push(Field {
                name: field_entry.name().to_string(),
                r#type: data_type.to_string(),
            })
        }

        Self(fields)
    }
}
pub(crate) fn detailed_field(index: &Index, field: String) {}
