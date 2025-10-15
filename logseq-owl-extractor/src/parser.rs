use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LogseqPage {
    pub title: String,
    pub properties: HashMap<String, Vec<String>>,
    pub owl_blocks: Vec<String>,
}

/// Parse a Logseq markdown file and extract properties and OWL blocks
pub fn parse_logseq_file(path: &Path) -> Result<LogseqPage> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read file: {}", path.display()))?;

    let title = extract_title(path, &content);
    let properties = extract_properties(&content);
    let owl_blocks = extract_owl_blocks(&content)?;

    Ok(LogseqPage {
        title,
        properties,
        owl_blocks,
    })
}

/// Extract the page title from the file path or first heading
fn extract_title(path: &Path, content: &str) -> String {
    // Try to find a level-1 heading
    let heading_re = Regex::new(r"^#\s+(.+)$").unwrap();
    for line in content.lines() {
        if let Some(cap) = heading_re.captures(line) {
            return cap[1].trim().to_string();
        }
    }

    // Fall back to filename without extension
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

/// Extract Logseq properties (key:: value format)
fn extract_properties(content: &str) -> HashMap<String, Vec<String>> {
    let mut properties = HashMap::new();
    let property_re = Regex::new(r"^([a-zA-Z][a-zA-Z0-9-_]*)::\s*(.+)$").unwrap();

    for line in content.lines() {
        if let Some(cap) = property_re.captures(line.trim()) {
            let key = cap[1].to_string();
            let value = cap[2].to_string();

            // Split on commas for multi-value properties
            let values: Vec<String> = value
                .split(',')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect();

            properties.entry(key).or_insert_with(Vec::new).extend(values);
        }
    }

    properties
}

/// Extract OWL Functional Syntax blocks
/// Supports two formats:
/// 1. Code fence format (Logseq outline):
///    ```
///    owl:functional-syntax:: |
///      Declaration(...)
///    ```
/// 2. Direct indented format:
///    owl:functional-syntax:: |
///      Declaration(...)
fn extract_owl_blocks(content: &str) -> Result<Vec<String>> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check for code fence containing owl:functional-syntax or clojure
        // Handle both direct fences and Logseq bullet fences (- ```clojure)
        let fence_match = if line.starts_with("```") {
            Some(line)
        } else if line.starts_with("- ```") {
            Some(&line[2..])  // Skip "- " prefix
        } else {
            None
        };

        if let Some(fence_line) = fence_match {
            let language = fence_line.trim_start_matches("```").trim();

            // Check if it's a clojure fence (OWL Functional Syntax)
            if language == "clojure" || language.is_empty() {
                i += 1;
                if i >= lines.len() {
                    break;
                }

                // For clojure fences, treat entire content as OWL
                // For empty fences, check if next line has owl:functional-syntax marker
                let should_extract = if language == "clojure" {
                    true
                } else if lines[i].trim().starts_with("owl:functional-syntax::") {
                    // Skip the owl:functional-syntax:: line if present
                    i += 1;
                    true
                } else {
                    false
                };

                if should_extract {
                    // Extract until closing ```
                    let mut block_lines = Vec::new();
                    while i < lines.len() {
                        let current_line = lines[i];
                        if current_line.trim().starts_with("```") {
                            break;
                        }
                        // Filter out Clojure-style comments and preserve code
                        let trimmed = current_line.trim_start();
                        if !trimmed.is_empty()
                            && !trimmed.starts_with(";;")
                            && !trimmed.starts_with("#")
                            && trimmed != "|" {
                            block_lines.push(trimmed);
                        }
                        i += 1;
                    }

                    // Only add if the block contains OWL syntax
                    let block_text = block_lines.join("\n");
                    let is_owl = block_text.contains("Declaration(")
                        || block_text.contains("SubClassOf(")
                        || block_text.contains("EquivalentClasses(")
                        || block_text.contains("DisjointClasses(")
                        || block_text.contains("ObjectProperty(")
                        || block_text.contains("DataProperty(");

                    if is_owl && !block_lines.is_empty() {
                        blocks.push(block_text);
                    }
                }
            }
            i += 1;
            continue;
        }

        // Original format: direct owl:functional-syntax:: |
        if line.starts_with("owl:functional-syntax::") {
            i += 1;
            if i >= lines.len() {
                break;
            }

            // Check if next line is the pipe character
            if !lines[i].trim().starts_with('|') {
                i += 1;
                continue;
            }

            i += 1;

            // Extract the indented block
            let mut block_lines = Vec::new();
            let base_indent = if i < lines.len() {
                lines[i].len() - lines[i].trim_start().len()
            } else {
                0
            };

            while i < lines.len() {
                let current_line = lines[i];
                let current_indent = current_line.len() - current_line.trim_start().len();

                // Stop if we hit a line that's not indented or is less indented than the base
                if !current_line.trim().is_empty() && current_indent < base_indent {
                    break;
                }

                // Stop if we hit another property, heading, or code fence
                if current_line.trim_start().starts_with('#')
                    || current_line.trim().starts_with("```")
                    || (current_line.contains("::") && !current_line.trim().starts_with("//"))
                {
                    break;
                }

                // Add non-empty lines, removing the base indentation
                if current_indent >= base_indent && !current_line.trim().is_empty() {
                    let trimmed = if current_indent >= base_indent {
                        &current_line[base_indent..]
                    } else {
                        current_line.trim_start()
                    };
                    block_lines.push(trimmed);
                }

                i += 1;
            }

            if !block_lines.is_empty() {
                blocks.push(block_lines.join("\n"));
            }
        } else {
            i += 1;
        }
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_properties() {
        let content = r#"
# Test Page

term-id:: 20067
maturity:: mature
has-part:: [[Visual Mesh]], [[Animation Rig]]
"#;

        let props = extract_properties(content);
        assert_eq!(props.get("term-id").unwrap()[0], "20067");
        assert_eq!(props.get("maturity").unwrap()[0], "mature");
        assert_eq!(props.get("has-part").unwrap().len(), 2);
    }

    #[test]
    fn test_extract_owl_blocks() {
        let content = r#"
owl:functional-syntax:: |
  Declaration(Class(mv:Avatar))
  SubClassOf(mv:Avatar mv:VirtualEntity)
"#;

        let blocks = extract_owl_blocks(content).unwrap();
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("Declaration(Class(mv:Avatar))"));
    }

    #[test]
    fn test_extract_owl_blocks_code_fence() {
        let content = r#"
	- ## OWL Axioms
	  collapsed:: true
		- ```
		  owl:functional-syntax:: |
		    Declaration(Class(mv:Avatar))

		    # Classification
		    SubClassOf(mv:Avatar mv:VirtualEntity)
		    SubClassOf(mv:Avatar mv:Agent)
		  ```
"#;

        let blocks = extract_owl_blocks(content).unwrap();
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("Declaration(Class(mv:Avatar))"));
        assert!(blocks[0].contains("SubClassOf(mv:Avatar mv:VirtualEntity)"));
        assert!(blocks[0].contains("SubClassOf(mv:Avatar mv:Agent)"));
    }

    #[test]
    fn test_extract_properties_from_outline() {
        let content = r#"
- OntologyBlock
  collapsed:: true
	- term-id:: 20067
	- preferred-term:: Avatar
	- owl:class:: mv:Avatar
	- owl:physicality:: VirtualEntity
	- owl:role:: Agent
"#;

        let props = extract_properties(content);
        assert_eq!(props.get("term-id").unwrap()[0], "20067");
        assert_eq!(props.get("preferred-term").unwrap()[0], "Avatar");
        assert_eq!(props.get("owl:class").unwrap()[0], "mv:Avatar");
        assert_eq!(props.get("owl:physicality").unwrap()[0], "VirtualEntity");
        assert_eq!(props.get("owl:role").unwrap()[0], "Agent");
    }
}
