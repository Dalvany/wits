use tantivy::schema::Schema;
use tantivy::space_usage::PerFieldSpaceUsage;
use tantivy::Index;
use tantivy::Result;

fn show_per_field(
    schema: &Schema,
    fields: &Vec<String>,
    r#type: &str,
    fields_usage: &PerFieldSpaceUsage,
) {
    println!("\t{}", r#type);
    println!(
        "\t\tTotal {} : {} bytes",
        r#type.to_ascii_lowercase(),
        fields_usage.total()
    );
    for (field, field_usage) in fields_usage.fields() {
        let field_name = schema.get_field_entry(*field).name();
        if fields.is_empty() || fields.contains(&field_name.to_string()) {
            println!(
                "\t\t'{}' {} : {} bytes",
                field_name,
                r#type.to_ascii_lowercase(),
                field_usage.total()
            );
        }
    }
}

pub fn show_space_usage(index: &Index, fields: Vec<String>) -> Result<()> {
    let searcher = index.reader()?.searcher();

    let space_usage = searcher.space_usage()?;
    let segment_usage = space_usage.segments();

    println!("Total usage : {} bytes", space_usage.total());
    println!("Number of segments : {}", segment_usage.len());

    let segments = searcher.segment_readers();

    let schema = &index.schema();

    for segment in segments {
        println!();
        println!("Segment {}", segment.segment_id().uuid_string());
        let segment_usage = segment.space_usage()?;
        println!("\tNum docs : {}", segment_usage.num_docs());
        println!("\tDeleted : {} bytes", segment_usage.deletes());

        // Term dict
        show_per_field(schema, &fields, "Term dict", segment_usage.termdict());

        // Fast field
        show_per_field(schema, &fields, "Fast field", segment_usage.fast_fields());

        // Posting
        show_per_field(schema, &fields, "Posting", segment_usage.postings());

        // Position
        show_per_field(schema, &fields, "Position", segment_usage.positions());

        // Fiels norms
        show_per_field(schema, &fields, "Field norms", segment_usage.fieldnorms());
    }
    Ok(())
}
