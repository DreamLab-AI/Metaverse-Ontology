# Metaverse Ontology - Hybrid Logseq + OWL Design

A formal ontology for metaverse concepts using an innovative hybrid approach that combines **Logseq markdown** for human readability with **OWL Functional Syntax** for formal reasoning.

## 🌟 Key Features

- **Orthogonal Classification**: Two-dimensional design (Physicality × Role) enabling automatic inference of 9 intersection classes
- **Hybrid Format**: Logseq properties for navigation + embedded OWL axioms for reasoning
- **Automated Extraction**: Rust tool to parse Logseq files and generate complete OWL ontology
- **OWL 2 DL Compliant**: Full support for formal reasoning and consistency checking
- **horned-owl Compatible**: Uses OWL Functional Syntax that can be directly parsed

## 📁 Project Structure

```
OntologyDesign/
├── README.md                          # This file
├── QUICKSTART.md                      # 5-minute setup guide
├── IMPLEMENTATION_STATUS.md           # Detailed status of all issues resolved
├── URIMapping.md                      # Wikilink → IRI conversion rules
│
├── OntologyDefinition.md             # Ontology header, base classes, axioms
├── PropertySchema.md                  # All object/data/annotation properties
├── ETSIDomainClassification.md       # ETSI domain taxonomy
├── ValidationTests.md                 # Test cases for reasoning
│
├── Avatar.md                          # Example: VirtualAgent class
├── DigitalTwin.md                     # Example: HybridObject class
│
└── logseq-owl-extractor/             # Rust extraction tool
    ├── Cargo.toml                     # Dependencies
    ├── README.md                      # Tool documentation
    └── src/
        ├── main.rs                    # CLI interface
        ├── parser.rs                  # Markdown parser
        ├── converter.rs               # Wikilink → IRI conversion
        └── assembler.rs               # Ontology assembly & validation
```

## 🚀 Quick Start

### 1. Build the Extractor

```bash
cd logseq-owl-extractor
cargo build --release
```

### 2. Extract OWL from Logseq Files

```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --validate
```

### 3. View in Protégé or Use with Reasoner

```bash
# Open in Protégé (GUI)
protege metaverse-ontology.ofn

# Or classify with whelk-rs (CLI)
whelk classify metaverse-ontology.ofn
```

📖 **See [QUICKSTART.md](QUICKSTART.md) for detailed instructions.**

## 🎯 Design Philosophy

### Orthogonal Classification

The ontology uses two independent dimensions:

| Physicality | Role | → Result |
|-------------|------|----------|
| Physical | Agent | **PhysicalAgent** (e.g., Human) |
| Virtual | Agent | **VirtualAgent** (e.g., Avatar) |
| Hybrid | Agent | **HybridAgent** (e.g., Cyborg) |
| Physical | Object | **PhysicalObject** (e.g., VR Headset) |
| Virtual | Object | **VirtualObject** (e.g., 3D Model) |
| Hybrid | Object | **HybridObject** (e.g., Digital Twin) |
| ... | ... | ... (9 total combinations) |

This allows:
- Natural multiple inheritance
- Automatic classification by reasoners
- Clean separation of concerns

### Hybrid Logseq + OWL Format

Each concept is defined in a Logseq markdown file with:

1. **Logseq Properties** (for humans):
   ```markdown
   term-id:: 20067
   has-part:: [[Visual Mesh]], [[Animation Rig]]
   requires:: [[3D Rendering Engine]]
   ```

2. **OWL Functional Syntax** (for reasoners):
   ```markdown
   owl:functional-syntax:: |
     Declaration(Class(mv:Avatar))
     SubClassOf(mv:Avatar mv:VirtualEntity)
     SubClassOf(mv:Avatar mv:Agent)
   ```

**Benefits:**
- ✅ Searchable and navigable in Logseq
- ✅ Formal reasoning with OWL 2 DL reasoners
- ✅ Version control friendly (plain markdown)
- ✅ Human-readable and machine-processable

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [QUICKSTART.md](QUICKSTART.md) | Get started in 5 minutes |
| [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) | Detailed status of all features |
| [URIMapping.md](URIMapping.md) | Wikilink to IRI conversion rules |
| [task.md](task.md) | Original requirements and design analysis |
| [logseq-owl-extractor/README.md](logseq-owl-extractor/README.md) | Extractor tool documentation |

## 🧪 Example Classes

### Avatar (VirtualAgent)

```markdown
# Avatar

owl:physicality-dimension:: VirtualEntity
owl:role-dimension:: Agent

owl:functional-syntax:: |
  Declaration(Class(mv:Avatar))
  SubClassOf(mv:Avatar mv:VirtualEntity)
  SubClassOf(mv:Avatar mv:Agent)
  SubClassOf(mv:Avatar
    ObjectExactCardinality(1 mv:represents mv:Agent)
  )
```

➡️ Reasoner infers: `Avatar ⊑ VirtualAgent`

### Digital Twin (HybridObject)

```markdown
# Digital Twin

owl:physicality-dimension:: HybridEntity
owl:role-dimension:: Object

owl:functional-syntax:: |
  Declaration(Class(mv:DigitalTwin))
  SubClassOf(mv:DigitalTwin mv:HybridEntity)
  SubClassOf(mv:DigitalTwin mv:Object)
  SubClassOf(mv:DigitalTwin
    ObjectExactCardinality(1 mv:synchronizesWith mv:PhysicalEntity)
  )
```

➡️ Reasoner infers: `DigitalTwin ⊑ HybridObject`

## ✅ Validation Tests

The ontology includes test cases for:

- ✅ **Inference**: Avatar → VirtualAgent classification
- ✅ **Consistency**: Valid Digital Twin with proper bindings
- ⚠️ **Inconsistency Detection**: Digital Twin missing required binding
- ❌ **Disjointness Violation**: Entity in both Physical and Virtual

See [ValidationTests.md](ValidationTests.md) for details.

## 🔧 Technology Stack

- **Format**: OWL 2 DL (Web Ontology Language)
- **Syntax**: OWL Functional Syntax
- **Parser**: horned-owl + horned-functional (Rust)
- **Reasoner**: Compatible with HermiT, Pellet, whelk-rs, etc.
- **Editor**: Logseq (or any markdown editor)
- **Viewer**: Protégé (optional)

## 🛠️ Extractor Tool Features

- 📝 Parse Logseq markdown with embedded OWL
- 🔗 Extract and combine OWL Functional Syntax blocks
- 🏗️ Assemble complete ontology document
- ✅ Validate syntax with horned-owl
- 🔄 Optional: Convert Logseq properties to OWL axioms
- 🎯 Wikilink → IRI conversion

## 📊 Current Status

| Component | Status |
|-----------|--------|
| Base Classes | ✅ Complete |
| Intersection Classes | ✅ Complete (9/9) |
| Object Properties | ✅ Complete |
| Data Properties | ✅ Complete |
| Example Classes | ✅ Complete (Avatar, DigitalTwin) |
| Validation Tests | ✅ Complete |
| Extraction Tool | ✅ Complete |
| Documentation | ✅ Complete |

**All issues from [task.md](task.md) have been resolved!** 🎉

## 🚧 Future Enhancements

- [ ] Integration with whelk-rs for automatic reasoning
- [ ] SWRL rule support for complex inference
- [ ] Export to RDF/XML, Turtle, Manchester Syntax
- [ ] Web interface for browsing the ontology
- [ ] GitHub Actions for continuous validation
- [ ] Incremental extraction (only changed files)

## 🤝 Contributing

Contributions welcome! To add a new concept:

1. Create a new `.md` file
2. Follow the format in [Avatar.md](Avatar.md) or [DigitalTwin.md](DigitalTwin.md)
3. Run the extractor to validate
4. Submit a PR

## 📖 Learn More

### OWL 2 Resources
- [OWL 2 Web Ontology Language Primer](https://www.w3.org/TR/owl2-primer/)
- [OWL 2 Functional Syntax](https://www.w3.org/TR/owl2-syntax/)
- [horned-owl documentation](https://docs.rs/horned-owl/)

### Metaverse Standards
- [ETSI GR MEC 039: Multi-access Edge Computing (MEC) Framework](https://www.etsi.org/)
- [ISO 23247: Automation systems and integration — Digital Twin framework](https://www.iso.org/)
- [Web3D Consortium - H-Anim](https://www.web3d.org/working-groups/humanoid-animation-h-anim)

### Tools
- [Protégé](https://protege.stanford.edu/) - Ontology editor
- [whelk-rs](https://github.com/balhoff/whelk) - Fast OWL 2 EL reasoner in Rust
- [ROBOT](https://github.com/ontodev/robot) - OWL tool for command line

## 📄 License

MIT or Apache 2.0 (choose one based on your requirements)

## 👥 Authors

Metaverse Ontology Working Group

## 🙏 Acknowledgments

- Built with [horned-owl](https://github.com/phillord/horned-owl) by Phillip Lord
- Inspired by the Logseq knowledge management approach
- Aligned with ETSI and ISO metaverse standards

---

**Ready to start?** See [QUICKSTART.md](QUICKSTART.md) for setup instructions!
