# Logseq OWL Extractor

A Rust tool that extracts OWL Functional Syntax from Logseq markdown files and assembles them into a complete, valid OWL 2 ontology.

## Features

- 📝 Parses Logseq markdown files with embedded OWL Functional Syntax
- 🔗 Extracts `owl:functional-syntax:: |` blocks from multiple files
- 🏗️ Assembles fragments into a complete OWL ontology document
- ✅ Validates ontology syntax using `horned-owl`
- 🔄 Optionally converts Logseq properties to OWL axioms
- 🎯 Handles wikilink → IRI conversion

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from source

```bash
cd logseq-owl-extractor
cargo build --release
```

The compiled binary will be at `target/release/logseq-owl-extractor`.

## Usage

### Basic extraction

Extract OWL from all markdown files in the current directory:

```bash
logseq-owl-extractor --input . --output ontology.ofn
```

### With property conversion

Also convert Logseq properties (like `has-part::`, `requires::`) to OWL axioms:

```bash
logseq-owl-extractor --input . --output ontology.ofn --convert-properties
```

### Skip validation

Skip validation step (useful for debugging):

```bash
logseq-owl-extractor --input . --output ontology.ofn --validate=false
```

## Input Format

### Logseq Markdown Files

Each `.md` file should follow this structure:

```markdown
# Concept Name

## Core Properties

term-id:: 20067
preferred-term:: Avatar
definition:: Digital representation of a person...
maturity:: mature

## Ontological Relationships

has-part:: [[Visual Mesh]], [[Animation Rig]]
requires:: [[3D Rendering Engine]]

## OWL Functional Syntax

owl:functional-syntax:: |
  Declaration(Class(mv:Avatar))
  SubClassOf(mv:Avatar mv:VirtualEntity)
  SubClassOf(mv:Avatar mv:Agent)
```

### Special Files

- **OntologyDefinition.md**: Must exist and contain the ontology header with prefix declarations and base axioms
- Other `.md` files: Contain class/property definitions

## Output Format

The tool generates a single OWL Functional Syntax (`.ofn`) file that can be:

- Imported into Protégé
- Parsed by `horned-owl`
- Reasoned over with `whelk-rs` or other OWL 2 DL reasoners
- Converted to RDF/XML, Turtle, or other formats

## Architecture

```
┌─────────────────────┐
│   Markdown Files    │
│  (.md in Logseq)    │
└──────────┬──────────┘
           │
           ▼
     ┌─────────┐
     │ Parser  │  Extracts properties & OWL blocks
     └────┬────┘
          │
          ▼
    ┌──────────┐
    │Converter │  Wikilink → IRI conversion
    └────┬─────┘
         │
         ▼
   ┌───────────┐
   │ Assembler │  Combines fragments
   └─────┬─────┘
         │
         ▼
   ┌───────────┐
   │ Validator │  horned-owl parsing
   └─────┬─────┘
         │
         ▼
  ┌──────────────┐
  │  ontology.ofn│  Complete OWL document
  └──────────────┘
```

## Module Overview

### `parser.rs`
- Parses Logseq markdown files
- Extracts properties (key:: value format)
- Extracts OWL Functional Syntax blocks

### `converter.rs`
- Converts Logseq wikilinks `[[Page Name]]` to IRIs
- Transforms kebab-case properties to camelCase
- Generates OWL axioms from Logseq properties

### `assembler.rs`
- Combines header from OntologyDefinition.md
- Adds axioms from all other files
- Generates complete OWL document
- Validates with horned-owl

### `main.rs`
- CLI interface (using `clap`)
- Orchestrates the extraction pipeline

## Testing

Run the test suite:

```bash
cargo test
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

## Example Workflow

1. Write your ontology in Logseq markdown files:
   - `OntologyDefinition.md` - Header and base classes
   - `Avatar.md` - Avatar class definition
   - `DigitalTwin.md` - Digital Twin definition
   - etc.

2. Extract to OWL:
   ```bash
   logseq-owl-extractor --input . --output metaverse-ontology.ofn
   ```

3. Validate the output:
   ```bash
   # The tool validates automatically, but you can also use external tools:
   # - Load in Protégé
   # - Run a DL reasoner (whelk-rs, HermiT, Pellet, etc.)
   ```

4. Convert to other formats (optional):
   ```bash
   # Using robot (https://github.com/ontodev/robot)
   robot convert --input metaverse-ontology.ofn --output ontology.owl
   robot convert --input metaverse-ontology.ofn --output ontology.ttl
   ```

## Dependencies

- **horned-owl** (0.11): OWL 2 data model implementation
- **horned-functional** (0.4): OWL Functional Syntax parser
- **regex** (1.10): Pattern matching for parsing
- **walkdir** (2.4): Recursive directory traversal
- **anyhow** (1.0): Error handling
- **clap** (4.5): CLI argument parsing

## Limitations

- Does not perform full DL reasoning (use external reasoner for that)
- Assumes well-formed OWL Functional Syntax in input files
- Wikilink conversion follows specific naming conventions (see URIMapping.md)

## Future Enhancements

- [ ] Integration with `whelk-rs` for full reasoning
- [ ] Export to RDF/XML, Turtle, Manchester Syntax
- [ ] Incremental extraction (only changed files)
- [ ] Property consistency checking
- [ ] SWRL rule support
- [ ] Web service API

## License

MIT or Apache 2.0 (choose one)

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## See Also

- [OWL 2 Web Ontology Language](https://www.w3.org/TR/owl2-overview/)
- [horned-owl documentation](https://docs.rs/horned-owl/)
- [Logseq](https://logseq.com/)
