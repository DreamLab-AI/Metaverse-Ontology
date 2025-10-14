use anyhow::Result;
use regex::Regex;

use crate::parser::LogseqPage;

/// Convert Logseq properties to OWL axioms
pub fn logseq_properties_to_owl(page: &LogseqPage) -> Result<Vec<String>> {
    let mut axioms = Vec::new();

    // Convert wikilink-based properties to OWL object property assertions
    for (property, values) in &page.properties {
        // Skip OWL-specific properties (these are metadata, not ontological assertions)
        if property.starts_with("owl:") || property.starts_with("term-") {
            continue;
        }

        // Skip definition, maturity, source, etc. (these are annotations)
        if matches!(
            property.as_str(),
            "definition"
                | "maturity"
                | "source"
                | "preferred-term"
                | "synonyms"
        ) {
            continue;
        }

        // Convert property name from kebab-case to camelCase
        let owl_property = kebab_to_camel(property);

        for value in values {
            // Check if value is a wikilink
            if let Some(linked_class) = extract_wikilink(value) {
                // Convert wikilink to IRI
                let class_iri = wikilink_to_iri(&linked_class);

                // Generate SubClassOf axiom with existential restriction
                let axiom = format!(
                    "SubClassOf(mv:{}\n  ObjectSomeValuesFrom(mv:{} mv:{}))",
                    wikilink_to_iri(&page.title),
                    owl_property,
                    class_iri
                );
                axioms.push(axiom);
            }
        }
    }

    // Convert data properties
    if let Some(maturity_values) = page.properties.get("maturity") {
        if let Some(maturity) = maturity_values.first() {
            let axiom = format!(
                "ClassAssertion(DataHasValue(mv:maturity \"{}\"^^xsd:string) mv:{})",
                maturity,
                wikilink_to_iri(&page.title)
            );
            axioms.push(axiom);
        }
    }

    if let Some(term_id_values) = page.properties.get("term-id") {
        if let Some(term_id) = term_id_values.first() {
            let axiom = format!(
                "ClassAssertion(DataHasValue(mv:termId {}^^xsd:integer) mv:{})",
                term_id,
                wikilink_to_iri(&page.title)
            );
            axioms.push(axiom);
        }
    }

    Ok(axioms)
}

/// Convert kebab-case to camelCase
fn kebab_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for ch in s.chars() {
        if ch == '-' || ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}

/// Extract the content of a wikilink [[Page Name]]
fn extract_wikilink(s: &str) -> Option<String> {
    let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    re.captures(s).map(|cap| cap[1].to_string())
}

/// Convert a wikilink or page title to an OWL IRI (without namespace prefix)
fn wikilink_to_iri(s: &str) -> String {
    // Remove brackets if present
    let cleaned = s.replace("[[", "").replace("]]", "");

    // Split into words, capitalize each, and join
    cleaned
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = String::new();
                    // Keep numbers and letters, capitalize first letter of each word
                    for ch in first.to_string().chars().chain(chars) {
                        if ch.is_alphanumeric() {
                            result.push(ch);
                        } else if ch == '-' {
                            // Convert hyphen to nothing (merge words)
                        } else {
                            result.push('_');
                        }
                    }
                    result
                }
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_to_camel() {
        assert_eq!(kebab_to_camel("has-part"), "hasPart");
        assert_eq!(kebab_to_camel("is-part-of"), "isPartOf");
        assert_eq!(kebab_to_camel("requires"), "requires");
    }

    #[test]
    fn test_extract_wikilink() {
        assert_eq!(
            extract_wikilink("[[Visual Mesh]]"),
            Some("Visual Mesh".to_string())
        );
        assert_eq!(
            extract_wikilink("[[Animation Rig]], [[Other]]"),
            Some("Animation Rig".to_string())
        );
        assert_eq!(extract_wikilink("not a link"), None);
    }

    #[test]
    fn test_wikilink_to_iri() {
        assert_eq!(wikilink_to_iri("Visual Mesh"), "VisualMesh");
        assert_eq!(wikilink_to_iri("Digital Twin"), "DigitalTwin");
        assert_eq!(wikilink_to_iri("3D Rendering Engine"), "3DRenderingEngine");
        assert_eq!(wikilink_to_iri("ACM + Web3D HAnim"), "ACM_Web3DHAnim");
    }
}
