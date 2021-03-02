use std::error::Error;
use yarner_lib::{Context, Document, Node};

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut data = yarner_lib::parse_input()?;
    let config = &data.context.config;

    check_version(&data.context);

    let placeholder = config
        .get("placeholder")
        .and_then(|s| s.as_str())
        .unwrap_or("[[_TOC_]]");

    let min_level = config
        .get("min-level")
        .and_then(|s| s.as_integer())
        .unwrap_or(2);

    let max_level = config
        .get("max-level")
        .and_then(|s| s.as_integer())
        .unwrap_or(5);

    for (_, mut doc) in data.documents.iter_mut() {
        let headings = extract_headings(&doc, min_level as usize, max_level as usize);
        replace_toc_marker(&mut doc, &headings, placeholder);
    }

    yarner_lib::write_output(&data)?;
    Ok(())
}

fn extract_headings(
    document: &Document,
    min_level: usize,
    max_level: usize,
) -> Vec<(String, usize)> {
    let mut headings: Vec<(String, usize)> = Vec::new();

    for node in document.nodes.iter() {
        if let Node::Text(block) = node {
            for line in &block.text {
                if let Some((heading, level)) = heading_level(&line) {
                    if level >= min_level && level <= max_level {
                        headings.push((heading.to_owned(), level - min_level));
                    }
                }
            }
        }
    }

    headings
}

fn replace_toc_marker(document: &mut Document, toc: &[(String, usize)], placeholder: &str) {
    for node in document.nodes.iter_mut() {
        if let Node::Text(block) = node {
            for line_idx in 0..block.text.len() {
                if block.text[line_idx].contains(placeholder) {
                    let toc_lines = generate_toc(&toc);
                    block.text = block
                        .text
                        .iter()
                        .take(line_idx)
                        .chain(toc_lines.iter())
                        .chain(block.text.iter().skip(line_idx + 1))
                        .cloned()
                        .collect();
                    break;
                }
            }
        }
    }
}

fn generate_toc(toc: &[(String, usize)]) -> Vec<String> {
    let mut result = vec![];

    for (heading, level) in toc {
        result.push(format!("{}* {}", "  ".repeat(*level), heading));
    }

    result
}

fn heading_level(line: &str) -> Option<(&str, usize)> {
    if line.starts_with('#') {
        let mut level = 1;
        while level < line.len() && line[level..].starts_with('#') {
            level += 1;
        }
        let heading = line[level..].trim();
        if heading.is_empty() {
            None
        } else {
            Some((heading, level))
        }
    } else {
        None
    }
}

fn check_version(context: &Context) {
    if context.yarner_version != yarner_lib::YARNER_VERSION {
        eprintln!(
            "  Warning: The {} plugin was built against version {} of Yarner, \
                    but we're being called from version {}",
            context.name,
            yarner_lib::YARNER_VERSION,
            context.yarner_version
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::heading_level;
    use yarner_lib::{Document, Node, TextBlock};

    #[test]
    fn no_heading() {
        assert_eq!(heading_level("Not a heading"), None);
    }

    #[test]
    fn no_label_heading() {
        assert_eq!(heading_level("###"), None);
    }

    #[test]
    fn heading_level_with_space() {
        assert_eq!(heading_level("## Heading"), Some(("Heading", 2)));
    }

    #[test]
    fn heading_level_without_space() {
        assert_eq!(heading_level("##Heading"), Some(("Heading", 2)));
    }

    #[test]
    fn generate_toc() {
        let toc = vec![("H1".to_string(), 0), ("H2".to_string(), 1)];
        let lines = super::generate_toc(&toc);

        assert_eq!(lines, vec!["* H1".to_string(), "  * H2".to_string()])
    }

    #[test]
    fn replace_marker() {
        let text = vec![
            "test".to_string(),
            "[[_TOC_]]".to_string(),
            "test".to_string(),
        ];
        let mut doc = Document {
            nodes: vec![Node::Text(TextBlock { text })],
            newline: "\n".to_string(),
        };
        let toc = vec![("H1".to_string(), 0), ("H2".to_string(), 1)];
        super::replace_toc_marker(&mut doc, &toc, "[[_TOC_]]");

        let expected = vec![
            "test".to_string(),
            "* H1".to_string(),
            "  * H2".to_string(),
            "test".to_string(),
        ];

        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Node::Text(_)));
        if let Node::Text(block) = &doc.nodes[0] {
            assert_eq!(block.text, expected);
        }
    }
}
