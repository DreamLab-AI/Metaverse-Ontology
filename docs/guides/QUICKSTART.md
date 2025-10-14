# Quick Start Guide

Get your Logseq + OWL ontology up and running in 5 minutes.

## Prerequisites

- Rust and Cargo (install from https://rustup.rs/)
- A Logseq vault with `.md` files containing OWL axioms

## Step 1: Build the Extractor

```bash
cd logseq-owl-extractor
cargo build --release
```

This compiles the extractor tool. The binary will be at `target/release/logseq-owl-extractor`.

## Step 2: Run the Extractor

From the project root:

```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --validate
```

This will:
- ✅ Find all `.md` files in the current directory
- ✅ Extract `owl:functional-syntax:: |` blocks
- ✅ Combine them into a single ontology file
- ✅ Validate the syntax using horned-owl

**Expected output:**
```
Logseq OWL Extractor v0.1.0
==============================
Input directory: .
Output file: metaverse-ontology.ofn

Found 6 markdown files
  ✓ Parsed: OntologyDefinition
  ✓ Parsed: PropertySchema
  ✓ Parsed: Avatar
  ✓ Parsed: DigitalTwin
  ✓ Parsed: ETSIDomainClassification
  ✓ Parsed: ValidationTests

Assembling ontology...
✓ Ontology written to metaverse-ontology.ofn

Validating ontology...
  ✓ Parsed successfully
  ✓ Ontology contains 87 axioms
  ℹ For full reasoning/consistency checking, use a DL reasoner like whelk-rs

Done!
```

## Step 3: Verify the Output

Check the generated file:

```bash
head -n 30 metaverse-ontology.ofn
```

You should see:
- Prefix declarations
- Ontology IRI
- Metadata annotations
- Class declarations
- Axioms from all your files

## Step 4: Load in Protégé (Optional)

If you have Protégé installed:

1. Open Protégé
2. File → Open
3. Select `metaverse-ontology.ofn`
4. Go to Reasoner → HermiT (or Pellet)
5. Click "Start reasoner"
6. Check the inferred class hierarchy

You should see:
- `Avatar` inferred as a subclass of `VirtualAgent`
- All 9 intersection classes properly classified
- No inconsistencies (unless you included test case 4)

## Step 5: Add New Concepts

Create a new concept file, e.g., `VRHeadset.md`:

```markdown
# VR Headset

## Core Properties

term-id:: 40001
preferred-term:: VR Headset
definition:: A head-mounted display device for virtual reality experiences.
maturity:: mature

## OWL Classification

owl:physicality-dimension:: PhysicalObject
owl:role-dimension:: Object

## OWL Functional Syntax

owl:functional-syntax:: |
  Declaration(Class(mv:VRHeadset))
  SubClassOf(mv:VRHeadset mv:PhysicalObject)
  SubClassOf(mv:VRHeadset mv:Hardware)
```

Re-run the extractor:

```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn
```

Your new class is now included!

## Step 6: Convert Properties (Optional)

To also generate OWL axioms from Logseq properties like `has-part::`:

```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --convert-properties
```

This will generate additional `SubClassOf` axioms with existential restrictions based on your Logseq wikilinks.

## Troubleshooting

### Error: "OntologyDefinition.md not found"

Make sure you have an `OntologyDefinition.md` file with the ontology header and prefix declarations.

### Error: "Failed to parse ontology"

Check that:
- All OWL Functional Syntax is valid
- Parentheses are balanced
- No typos in class/property names
- Prefixes are declared in OntologyDefinition.md

### No OWL blocks extracted

Make sure your blocks use the exact format:

```markdown
owl:functional-syntax:: |
  Declaration(Class(mv:Example))
  SubClassOf(mv:Example mv:Parent)
```

Note:
- Line must start with `owl:functional-syntax::`
- Must have ` |` (pipe) after the property name
- Content must be indented on following lines

## Next Steps

- 📖 Read [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for full details
- 📖 Check [URIMapping.md](URIMapping.md) for wikilink conversion rules
- 🧪 Review [ValidationTests.md](ValidationTests.md) for test cases
- 🔧 Customize the extractor in [logseq-owl-extractor/src/](logseq-owl-extractor/src/)

## Common Use Cases

### Export to RDF/XML

```bash
# Using robot tool (https://github.com/ontodev/robot)
robot convert \
  --input metaverse-ontology.ofn \
  --output metaverse-ontology.owl
```

### Export to Turtle

```bash
robot convert \
  --input metaverse-ontology.ofn \
  --output metaverse-ontology.ttl
```

### Run Reasoning with whelk-rs

```bash
# Install whelk-rs
cargo install whelk

# Run reasoning
whelk classify metaverse-ontology.ofn
```

### Continuous Integration

Add to your GitHub Actions workflow:

```yaml
name: Validate Ontology

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build extractor
        run: cd logseq-owl-extractor && cargo build --release
      - name: Extract and validate
        run: |
          ./logseq-owl-extractor/target/release/logseq-owl-extractor \
            --input . \
            --output ontology.ofn \
            --validate
```

## Need Help?

- 📚 Check the [README](logseq-owl-extractor/README.md)
- 📝 Review [task.md](task.md) for design rationale
- 🐛 Found a bug? Open an issue
- 💡 Have an idea? Submit a PR

Happy ontology building! 🎉
