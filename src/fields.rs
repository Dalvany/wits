use tantivy::schema::FieldType;
use tantivy::Index;

pub(crate) fn list_field(index: &Index) {
    let schema = index.schema();

    println!("Available fields");
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
        println!("Field name '{}' ({})", field_entry.name(), data_type);
    }
}

pub(crate) fn detailed_field(index: &Index, field: String) {}
