use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod parser;
mod converter;
mod assembler;

use assembler::OntologyAssembler;

/// Logseq OWL Extractor - Extracts OWL Functional Syntax from Logseq markdown files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing Logseq markdown files
    #[arg(short, long)]
    input: PathBuf,

    /// Output OWL Functional Syntax file
    #[arg(short, long)]
    output: PathBuf,

    /// Also convert Logseq properties to OWL axioms
    #[arg(short, long, default_value = "false")]
    convert_properties: bool,

    /// Validate the ontology after extraction
    #[arg(short, long, default_value = "true")]
    validate: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Logseq OWL Extractor v0.1.0");
    println!("==============================");
    println!("Input directory: {}", args.input.display());
    println!("Output file: {}", args.output.display());
    println!();

    // Step 1: Find all markdown files
    let markdown_files = find_markdown_files(&args.input)?;
    println!("Found {} markdown files", markdown_files.len());

    // Step 2: Parse each file
    let mut pages = Vec::new();
    for file_path in markdown_files {
        match parser::parse_logseq_file(&file_path) {
            Ok(page) => {
                println!("  ✓ Parsed: {}", page.title);
                pages.push(page);
            }
            Err(e) => {
                eprintln!("  ✗ Failed to parse {}: {}", file_path.display(), e);
            }
        }
    }
    println!();

    // Step 3: Assemble the ontology
    println!("Assembling ontology...");
    let mut assembler = OntologyAssembler::new();

    // Find and set the ontology definition (header)
    if let Some(header_page) = pages.iter().find(|p| p.title == "OntologyDefinition") {
        assembler.set_header(&header_page.owl_blocks)?;
    } else {
        anyhow::bail!("OntologyDefinition.md not found. This file must exist and contain the ontology header.");
    }

    // Add all other OWL blocks
    for page in &pages {
        if page.title != "OntologyDefinition" {
            assembler.add_owl_blocks(&page.owl_blocks)?;
        }
    }

    // Optionally convert Logseq properties to OWL axioms
    if args.convert_properties {
        println!("Converting Logseq properties to OWL axioms...");
        for page in &pages {
            let axioms = converter::logseq_properties_to_owl(page)?;
            assembler.add_axioms(&axioms)?;
        }
    }

    // Step 4: Write the combined ontology
    let ontology_text = assembler.to_string();
    fs::write(&args.output, ontology_text)
        .context("Failed to write output file")?;
    println!("✓ Ontology written to {}", args.output.display());

    // Step 5: Validate if requested
    if args.validate {
        println!();
        println!("Validating ontology...");
        match assembler.validate() {
            Ok(()) => println!("✓ Ontology is valid and consistent"),
            Err(e) => {
                eprintln!("✗ Validation failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!();
    println!("Done!");
    Ok(())
}

fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            files.push(path.to_path_buf());
        }
    }
    Ok(files)
}
