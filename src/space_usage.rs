use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use tantivy::schema::Schema;
use tantivy::space_usage::PerFieldSpaceUsage;
use tantivy::Index;

/// Hold usage for a field and a data structure (see [SegmentComponent](tantivy::SegmentComponent))
/// It contains the field name, the total size in byte and the percentage of usage relative to
/// the total size of index.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FieldUsage {
    /// Field name.
    field_name: String,
    /// Total size in byte for this field in the containing [SegmentComponent](tantivy::SegmentComponent).
    total_usage: usize,
    /// Percentage relative to the total size of index.
    percent_usage: usize,
}

impl Display for FieldUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\t\t'{}' : {} bytes ({}%)",
            self.field_name, self.total_usage, self.percent_usage
        )
    }
}

/// Disk usage for a [SegmentComponent](tantivy::SegmentComponent) in a [segment](tantivy::Segment).
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ComponentUsage {
    /// Name of the component.
    struct_name: String,
    /// Total size in byte of the component.
    total_usage: usize,
    /// Percentage relative to the total size of the index.
    percent_usage: usize,
    /// Fields size.
    fields: Vec<FieldUsage>,
}

impl Display for ComponentUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t{} :", self.struct_name)?;
        writeln!(
            f,
            "\t\tTotal : {} bytes ({}%)",
            self.total_usage, self.percent_usage
        )?;
        for fu in &self.fields {
            write!(f, "{}", fu)?;
        }

        Ok(())
    }
}

/// Size of a [Segment](tantivy::Segment) inside an [index](tantivy::Index).
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SegmentUsage {
    /// UUID of the segment.
    name: String,
    /// Size in byte of the segment
    total_usage: usize,
    /// Percentage size relative to the index size.
    percent_usage: usize,
    /// Num doc.
    num_doc: u32,
    /// Deleted.
    deleted_usage: usize,
    /// Components.
    components: Vec<ComponentUsage>,
}

impl Display for SegmentUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Segment {}", self.name)?;
        writeln!(
            f,
            "\tTotal : {} bytes ({}%)",
            self.total_usage, self.percent_usage
        )?;
        writeln!(f, "\tNum docs : {}", self.num_doc)?;
        writeln!(f, "\tDeleted : {} bytes", self.deleted_usage)?;
        for tu in &self.components {
            write!(f, "{}", tu)?;
        }
        Ok(())
    }
}

/// Disk usage of an [index](tantivy::Index).
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DiskUsage {
    /// Total size of the index.
    total_usage: usize,
    /// Segments.
    segments: Vec<SegmentUsage>,
}

impl Display for DiskUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Total : {} bytes", self.total_usage)?;
        writeln!(f, "Number of segments : {}", self.segments.len())?;
        for su in &self.segments {
            write!(f, "{}", su)?;
        }

        Ok(())
    }
}

impl DiskUsage {
    /// Create a new disk usage.
    ///
    /// # Parameters
    ///
    /// * index : Tantivy index
    /// * fields : list of fields to restrict to. If [None] or an empty vector is provided then
    /// all fields statistics will be displayed.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use tantivy::Index;
    /// use wits::DiskUsage;
    ///
    /// # fn main() -> std::result::Result<(), tantivy::TantivyError> {
    /// let index = Index::open_in_dir("/tmp/tantivy")?;
    /// let disk_usage = DiskUsage::new(&index, None)?;
    ///
    /// println!("{disk_usage}");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new(index: &Index, fields: Option<Vec<String>>) -> Result<Self, tantivy::TantivyError> {
        let fields = fields.unwrap_or_default();
        let searcher = index.reader()?.searcher();

        let space_usage = searcher.space_usage()?;
        let total = space_usage.total();

        let schema = index.schema();
        let segments = searcher.segment_readers();
        let mut sus: Vec<SegmentUsage> = Vec::with_capacity(segments.len());
        for segment in segments {
            let mut components: Vec<ComponentUsage> = Vec::with_capacity(5);
            let uuid = segment.segment_id().uuid_string();
            let segment_usage = segment.space_usage()?;
            let total_segment = segment_usage.total();
            let num_docs = segment_usage.num_docs();
            let deleted = segment_usage.deletes();
            let percent_segement = (segment_usage.total() * 100) / total;
            components.push(per_field_usage(
                &schema,
                total,
                &fields,
                "Positions",
                segment_usage.positions(),
            ));
            components.push(per_field_usage(
                &schema,
                total,
                &fields,
                "Posting",
                segment_usage.postings(),
            ));
            components.push(per_field_usage(
                &schema,
                total,
                &fields,
                "Fast fields",
                segment_usage.fast_fields(),
            ));
            components.push(per_field_usage(
                &schema,
                total,
                &fields,
                "Term dict",
                segment_usage.termdict(),
            ));
            components.push(per_field_usage(
                &schema,
                total,
                &fields,
                "Field norm",
                segment_usage.fieldnorms(),
            ));
            sus.push(SegmentUsage {
                name: uuid,
                total_usage: total_segment,
                percent_usage: percent_segement,
                num_doc: num_docs,
                deleted_usage: deleted,
                components,
            })
        }

        Ok(Self {
            total_usage: total,
            segments: sus,
        })
    }
}

fn per_field_usage(
    schema: &Schema,
    total_size: usize,
    fields: &Vec<String>,
    component: &str,
    fields_usage: &PerFieldSpaceUsage,
) -> ComponentUsage {
    let s = if fields.is_empty() {
        fields_usage.fields().count()
    } else {
        fields.len()
    };
    let mut fus: Vec<FieldUsage> = Vec::with_capacity(s);
    for (field, field_usage) in fields_usage.fields() {
        let field_name = schema.get_field_name(*field).to_string();
        if fields.is_empty() || fields.contains(&field_name) {
            let percent = (field_usage.total() * 100) / total_size;
            let fu = FieldUsage {
                field_name,
                total_usage: fields_usage.total(),
                percent_usage: percent,
            };
            fus.push(fu);
        }
    }

    let percent = (fields_usage.total() * 100) / total_size;
    ComponentUsage {
        struct_name: component.to_string(),
        total_usage: fields_usage.total(),
        percent_usage: percent,
        fields: fus,
    }
}
