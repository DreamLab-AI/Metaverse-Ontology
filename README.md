# Metaverse Ontology - Hybrid Logseq + OWL Design

A formal ontology for metaverse concepts using an innovative hybrid approach that combines **Logseq markdown** for human readability with **OWL Functional Syntax** for formal reasoning.

## 🌟 Key Features

- **Orthogonal Classification**: Two-dimensional design (Physicality × Role) enabling automatic inference of 9 intersection classes
- **Logseq-Native Format**: Pure outline format with collapsed blocks, queryable properties, and section IDs
- **Automated Extraction**: Rust tool to parse Logseq files and generate complete OWL ontology
- **OWL 2 DL Compliant**: Full support for formal reasoning and consistency checking
- **Dual Purpose**: Human-readable in Logseq, machine-extractable for reasoners
- **ETSI Aligned**: Domain classification based on ETSI metaverse standards

## 📁 Project Structure

```
OntologyDesign/
├── README.md                          # This file
│
├── docs/                              # 📚 Documentation
│   ├── guides/
│   │   ├── QUICKSTART.md             # 5-minute setup guide
│   │   └── MIGRATION_GUIDE.md        # How to migrate concepts
│   ├── reference/
│   │   ├── TEMPLATE.md               # Standard concept format template
│   │   ├── FORMAT_STANDARDIZED.md    # Complete format specification
│   │   ├── URIMapping.md             # Wikilink → IRI conversion rules
│   │   └── LOGSEQ_TAG_USAGE.md       # Using metaverseOntology tag
│   ├── IMPLEMENTATION_STATUS.md      # Feature completion status
│   ├── MIGRATION_STATUS.md           # 260+ concept migration progress
│   └── task.md                       # Original design requirements
│
├── OntologyDefinition.md             # 🎯 Core ontology header & base classes
├── PropertySchema.md                  # 🔗 All object/data/annotation properties
├── ETSIDomainClassification.md       # 🏛️ ETSI domain taxonomy
├── ValidationTests.md                 # ✅ Test cases for reasoning
│
├── Avatar.md                          # 📘 Example: VirtualAgent class
├── DigitalTwin.md                     # 📗 Example: HybridObject class
│
├── VisioningLab/                      # 🔬 260+ concept files
│   ├── Game Engine.md                # Example: VirtualObject class
│   └── [...]                         # (migration in progress)
│
└── logseq-owl-extractor/             # 🦀 Rust extraction tool
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

📖 **See [docs/guides/QUICKSTART.md](docs/guides/QUICKSTART.md) for detailed instructions.**

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

### Logseq-Native Format

Each concept is defined in a pure Logseq outline format:

```markdown
- ### OntologyBlock
  id:: avatar-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20067
	- preferred-term:: Avatar
	- definition:: Digital representation of a person...
	- owl:class:: mv:Avatar
	- owl:physicality:: VirtualEntity
	- owl:role:: Agent
	- owl:inferred-class:: mv:VirtualAgent
	- #### Relationships
	  id:: avatar-relationships
		- has-part:: [[Visual Mesh]], [[Animation Rig]]
		- requires:: [[3D Rendering Engine]]
	- #### OWL Axioms
	  id:: avatar-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:Avatar))
		  SubClassOf(mv:Avatar mv:VirtualEntity)
		  SubClassOf(mv:Avatar mv:Agent)
		  ```
- ## About Avatars
	- Human-readable description and examples...
```

**Benefits:**
- ✅ **Tidy**: Everything collapses into ### OntologyBlock
- ✅ **Queryable**: `metaverseOntology:: true` tag enables Logseq queries
- ✅ **Referenceable**: Section IDs allow block references
- ✅ **Readable**: Clojure syntax highlighting for OWL code
- ✅ **Extractable**: Parser extracts properties and OWL blocks
- ✅ **Documented**: Human context in "About" section
- ✅ **Linked**: WikiLinks create knowledge graph

## 📚 Documentation

### Getting Started
| Document | Description |
|----------|-------------|
| [QUICKSTART.md](docs/guides/QUICKSTART.md) | Get started in 5 minutes |
| [MIGRATION_GUIDE.md](docs/guides/MIGRATION_GUIDE.md) | How to migrate VisioningLab concepts |

### Reference
| Document | Description |
|----------|-------------|
| [TEMPLATE.md](docs/reference/TEMPLATE.md) | Standard concept format template |
| [FORMAT_STANDARDIZED.md](docs/reference/FORMAT_STANDARDIZED.md) | Complete format specification |
| [URIMapping.md](docs/reference/URIMapping.md) | Wikilink to IRI conversion rules |
| [LOGSEQ_TAG_USAGE.md](docs/reference/LOGSEQ_TAG_USAGE.md) | Using metaverseOntology tag for queries |

### Additional Resources
| Document | Description |
|----------|-------------|
| [CONSOLIDATED_KNOWLEDGEBASE.md](docs/CONSOLIDATED_KNOWLEDGEBASE.md) | Complete project overview |
| [FORWARD_IMPLEMENTATION_PLAN.md](docs/FORWARD_IMPLEMENTATION_PLAN.md) | Future enhancements roadmap |
| [SOLUTION_ARCHITECTURE_STRATEGY.md](docs/SOLUTION_ARCHITECTURE_STRATEGY.md) | Technical architecture details |

### Tools
| Document | Description |
|----------|-------------|
| [logseq-owl-extractor/README.md](logseq-owl-extractor/README.md) | Extractor tool documentation |

## 🧪 Example Concepts

### [Avatar.md](Avatar.md) - VirtualAgent

**Classification:**
- `owl:physicality:: VirtualEntity`
- `owl:role:: Agent`
- `owl:inferred-class:: mv:VirtualAgent` ✅

**Key Properties:**
- Represents exactly one user or AI agent
- Requires 3D Rendering Engine
- Enables User Embodiment and Social Presence
- Has parts: Visual Mesh, Animation Rig

**OWL Axioms:**
```owl
Declaration(Class(mv:Avatar))
SubClassOf(mv:Avatar mv:VirtualEntity)
SubClassOf(mv:Avatar mv:Agent)
SubClassOf(mv:Avatar
  ObjectExactCardinality(1 mv:represents mv:Agent)
)
```

➡️ **Reasoner infers:** `Avatar ⊑ VirtualAgent`

---

### [DigitalTwin.md](DigitalTwin.md) - HybridObject

**Classification:**
- `owl:physicality:: HybridEntity`
- `owl:role:: Object`
- `owl:inferred-class:: mv:HybridObject` ✅

**Key Properties:**
- Binds physical entity to virtual representation
- Synchronizes data in real-time
- Requires IoT sensors and data streams
- Implemented in Infrastructure Layer

**OWL Axioms:**
```owl
Declaration(Class(mv:DigitalTwin))
SubClassOf(mv:DigitalTwin mv:HybridEntity)
SubClassOf(mv:DigitalTwin mv:Object)
SubClassOf(mv:DigitalTwin
  ObjectExactCardinality(1 mv:synchronizesWith mv:PhysicalEntity)
)
```

➡️ **Reasoner infers:** `DigitalTwin ⊑ HybridObject`

---

### [VisioningLab/Game Engine.md](VisioningLab/Game%20Engine.md) - VirtualObject

**Classification:**
- `owl:physicality:: VirtualEntity`
- `owl:role:: Object`
- `owl:inferred-class:: mv:VirtualObject` ✅

**Key Properties:**
- Software platform for real-time 3D experiences
- Has parts: Rendering Pipeline, Physics Engine, Asset Manager
- Belongs to InfrastructureDomain and CreativeMediaDomain

➡️ **Reasoner infers:** `GameEngine ⊑ VirtualObject`

## ✅ Validation Tests

The ontology includes test cases for:

- ✅ **Inference**: Avatar → VirtualAgent classification
- ✅ **Consistency**: Valid Digital Twin with proper bindings
- ⚠️ **Inconsistency Detection**: Digital Twin missing required binding
- ❌ **Disjointness Violation**: Entity in both Physical and Virtual

See [ValidationTests.md](ValidationTests.md) for details.

## 💡 Use Cases

### 1. Metaverse Interoperability Standards
- Define common vocabulary for cross-platform metaverse systems
- Enable semantic interoperability between virtual worlds
- Support ETSI and ISO metaverse standardization efforts

### 2. Knowledge Graph Construction
- Build queryable knowledge base of metaverse concepts
- Link related concepts through formal relationships
- Support semantic search and discovery

### 3. Application Development
- Reference ontology for metaverse platform architects
- Semantic validation of system designs
- Documentation and communication tool for teams

### 4. Research & Analysis
- Classify emerging metaverse technologies
- Track evolution of metaverse concepts over time
- Identify gaps and opportunities in the metaverse stack

### 5. Automated Reasoning
- Automatically infer concept classifications
- Detect inconsistencies in system designs
- Validate architectural constraints

## 🔧 Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Ontology Language** | OWL 2 DL | Formal knowledge representation |
| **Syntax** | OWL Functional Syntax | Machine-readable axioms |
| **Source Format** | Logseq Markdown | Human-readable editing |
| **Parser/Validator** | horned-owl (Rust) | OWL parsing and validation |
| **Reasoner** | HermiT, Pellet, whelk-rs | Automatic classification and consistency checking |
| **Knowledge Editor** | Logseq | Visual outliner for navigation |
| **Ontology Viewer** | Protégé | GUI for visualizing class hierarchy |
| **Extraction Tool** | Custom Rust tool | Convert Logseq → OWL |
| **Version Control** | Git + Markdown | Track changes over time |

## 🛠️ Extractor Tool Features

The `logseq-owl-extractor` Rust tool provides:

- 📝 **Parse Logseq markdown** - Extracts properties and OWL blocks from outline format
- 🔗 **Combine OWL blocks** - Assembles axioms from multiple concept files
- 🏗️ **Build complete ontology** - Includes header from OntologyDefinition.md
- ✅ **Validate syntax** - Uses horned-owl to check OWL 2 DL compliance
- 🔄 **Convert wikilinks** - Transforms `[[Page Name]]` to `mv:PageName` IRIs
- 🎯 **Property conversion** - Optional conversion of Logseq properties to OWL axioms
- 📊 **Two format support** - Handles both code fence and direct indented OWL blocks
- 🔍 **Error reporting** - Clear messages for syntax errors with line numbers

**Example usage:**
```bash
logseq-owl-extractor --input . --output ontology.ofn --validate
```

See [logseq-owl-extractor/README.md](logseq-owl-extractor/README.md) for full documentation.

## 📊 Current Status

**Production Ready** - Full 281-concept metaverse ontology with OWL 2 DL compliance

| Component | Status |
|-----------|--------|
| Metaverse Concepts | ✅ 281 concepts fully migrated |
| OWL Validation | ✅ Zero errors, OWL 2 DL compliant |
| Multi-format Export | ✅ OWL/XML, Turtle, JSON-LD |
| Visualization | ✅ WebVOWL ready |
| Documentation | ✅ Complete |

## 🤝 Contributing

Contributions welcome! To add a new concept:

1. **Use the template**: Copy [docs/reference/TEMPLATE.md](docs/reference/TEMPLATE.md)
2. **Follow exemplars**: Study [Avatar.md](Avatar.md), [DigitalTwin.md](DigitalTwin.md), or [VisioningLab/Game Engine.md](VisioningLab/Game%20Engine.md)
3. **Classify correctly**: Use Physicality × Role dimensions (see [docs/guides/MIGRATION_GUIDE.md](docs/guides/MIGRATION_GUIDE.md))
4. **Add metaverseOntology tag**: Include `metaverseOntology:: true` property
5. **Validate**: Run the extractor to check OWL syntax
6. **Submit PR**: Include description of concept and its classification

### Adding New Concepts - Quick Checklist

- [ ] Filename matches concept name (spaces OK)
- [ ] ### OntologyBlock heading with `collapsed:: true`
- [ ] `metaverseOntology:: true` is first property
- [ ] Unique term-id assigned
- [ ] Clear definition provided
- [ ] owl:physicality dimension correct (Physical/Virtual/Hybrid)
- [ ] owl:role dimension correct (Agent/Object/Process)
- [ ] owl:inferred-class matches physicality + role
- [ ] At least one ETSI domain assigned
- [ ] OWL Axioms in code fence with ```clojure syntax
- [ ] File extracts successfully with logseq-owl-extractor
- [ ] Human-readable "About" section included

See [docs/reference/TEMPLATE.md](docs/reference/TEMPLATE.md) for full validation checklist.

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

---

## 📄 Licence

This project is licensed under the Mozilla Public Licence 2.0. See the [LICENSE](LICENSE) file for details.

---

## 👥 Authors

[John O'Hare](https://www.linkedin.com/in/jjohare/) for [DreamLab - AI Ltd](https://dreamlab-ai.com/) based on [VisioningLab](https://www.visioninglab.com/) [open Metaverse Ontology](/VisioningLab) 

## 🙏 Acknowledgments

- Built with [horned-owl](https://github.com/phillord/horned-owl) by Phillip Lord
- Inspired by the Logseq knowledge management approach
- Aligned with ETSI and ISO metaverse standards

---

## 🎉 Ready to Explore?

**Visualize the Ontology:**
1. Download [metaverse-ontology-webvowl.owl](visualization/metaverse-ontology-webvowl.owl)
2. Visit [WebVOWL](http://www.visualdataweb.de/webvowl/)
3. Upload and explore 281+ metaverse concepts interactively!

**Additional Documentation:**
- [Complete Knowledge Base](docs/CONSOLIDATED_KNOWLEDGEBASE.md) - Detailed project overview
- [Implementation Roadmap](docs/FORWARD_IMPLEMENTATION_PLAN.md) - Future enhancements
- [Architecture Details](docs/SOLUTION_ARCHITECTURE_STRATEGY.md) - Technical design

---


