use anyhow::Result;

/// Assembles OWL Functional Syntax fragments into a complete ontology
pub struct OntologyAssembler {
    header: String,
    axiom_blocks: Vec<String>,
}

impl OntologyAssembler {
    pub fn new() -> Self {
        Self {
            header: String::new(),
            axiom_blocks: Vec::new(),
        }
    }

    /// Set the ontology header from OntologyDefinition.md
    pub fn set_header(&mut self, owl_blocks: &[String]) -> Result<()> {
        if owl_blocks.is_empty() {
            anyhow::bail!("No OWL blocks found in ontology definition");
        }

        // Combine all blocks from the definition file
        self.header = owl_blocks.join("\n\n");
        Ok(())
    }

    /// Add OWL axiom blocks from other files
    pub fn add_owl_blocks(&mut self, owl_blocks: &[String]) -> Result<()> {
        for block in owl_blocks {
            if !block.trim().is_empty() {
                self.axiom_blocks.push(block.clone());
            }
        }
        Ok(())
    }

    /// Add axioms generated from Logseq properties
    pub fn add_axioms(&mut self, axioms: &[String]) -> Result<()> {
        for axiom in axioms {
            if !axiom.trim().is_empty() {
                self.axiom_blocks.push(axiom.clone());
            }
        }
        Ok(())
    }

    /// Generate the complete OWL Functional Syntax document
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        // Add the header (which should already be wrapped in Ontology(...))
        // We need to remove the closing parenthesis, add axioms, then close it

        let header = self.header.trim();

        // Check if header ends with the ontology closing parenthesis
        if header.ends_with(')') {
            // Remove the last closing parenthesis
            let header_without_close = &header[..header.len() - 1];
            result.push_str(header_without_close);
            result.push('\n');
        } else {
            result.push_str(header);
            result.push('\n');
        }

        // Add all axiom blocks with proper indentation
        for block in &self.axiom_blocks {
            result.push('\n');
            // Add indentation to each line
            for line in block.lines() {
                if !line.trim().is_empty() {
                    result.push_str("  ");
                    result.push_str(line);
                    result.push('\n');
                }
            }
        }

        // Close the ontology
        result.push_str(")\n");

        result
    }

    /// Validate the ontology using horned-owl
    pub fn validate(&self) -> Result<()> {
        use horned_functional::from_str;
        use horned_owl::ontology::set::SetOntology;

        let ontology_text = self.to_string();

        // Try to parse with horned-functional
        match from_str::<SetOntology, _>(&ontology_text) {
            Ok((_ontology, _prefixes)) => {
                println!("  ✓ Parsed successfully");
                println!("  ✓ OWL Functional Syntax is valid");

                // Basic consistency checks
                // Note: For full reasoning, you'd need to integrate with whelk or another reasoner
                println!("  ℹ For full reasoning/consistency checking, use a DL reasoner like whelk-rs");

                Ok(())
            }
            Err(e) => {
                anyhow::bail!("Failed to parse ontology: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_assembly() {
        let mut assembler = OntologyAssembler::new();

        let header = vec![
            r#"Prefix(mv:=<https://metaverse-ontology.org/>)
Ontology(<https://metaverse-ontology.org/>
  Declaration(Class(mv:Entity))
)"#
            .to_string(),
        ];

        assembler.set_header(&header).unwrap();

        let axioms = vec!["Declaration(Class(mv:Avatar))".to_string()];

        assembler.add_owl_blocks(&axioms).unwrap();

        let result = assembler.to_string();
        assert!(result.contains("Declaration(Class(mv:Entity))"));
        assert!(result.contains("Declaration(Class(mv:Avatar))"));
    }
}
