//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct RustHighlighter;

impl dialect::Highlight for RustHighlighter {
    fn highlight(&self, input: &str) -> Vec<dialect::HighlightedSpan> {
        let (analysis, file_id) = ra_ide::Analysis::from_single_file(input.to_string());

        analysis
            .highlight(file_id)
            .unwrap()
            .into_iter()
            .filter_map(
                |ra_ide::HighlightedRange {
                     range,
                     highlight: ra_ide::Highlight { tag, .. },
                     ..
                 }| {
                    let group = match tag {
                        ra_ide::HighlightTag::Function => {
                            Some(dialect::HighlightGroup::FunctionDef)
                        }
                        ra_ide::HighlightTag::Local => Some(dialect::HighlightGroup::VariableUse),
                        ra_ide::HighlightTag::Keyword => {
                            Some(dialect::HighlightGroup::OtherKeyword)
                        }
                        _ => None,
                    };

                    group.map(|group| dialect::HighlightedSpan {
                        range: range.start().into()..range.end().into(),
                        group,
                    })
                },
            )
            .collect()
    }
}
