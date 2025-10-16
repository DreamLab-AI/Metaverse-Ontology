# Metaverse Ontology

A formal ontology for metaverse concepts using an innovative hybrid approach that combines **Logseq markdown** for human readability with **OWL Functional Syntax** for formal reasoning.

## 🌟 Key Features

*   **Orthogonal Classification**: A two-dimensional design (Physicality × Role) enables the automatic inference of 9 intersection classes.
*   **Logseq-Native Format**: Concepts are defined in a pure outline format with properties, queryable tags, and collapsible blocks, making them easy to read and manage in Logseq.
*   **Automated Extraction**: A Rust-based tool parses the Logseq markdown files and generates a complete, valid OWL ontology.
*   **OWL 2 DL Compliant**: The generated ontology fully supports formal reasoning and consistency checking with standard OWL 2 DL tools.
*   **ETSI Aligned**: The domain classification is based on the European Telecommunications Standards Institute (ETSI) metaverse standards.

## 🎨 Visualization

The ontology can be explored visually using modern web-based tools that support the Turtle (.ttl) format.

![Ontology Visualization](docs/Screenshot%202025-10-15%20132107.png)
![Ontology Visualization](docs/Screenshot%202025-10-15%20144107.png)
![Ontology Visualization](docs/Screenshot%202025-10-16%20130730.png)
![Ontology Visualization](docs/Screenshot%202025-10-16%20130809.png)

### Recommended Visualizers

*   **OWL TTL Web Visualizer**: An interactive 3D force-directed graph visualizer that runs in your web browser. This tool uses Neo4j to render the ontology graph, allowing you to explore nodes and their connections dynamically. See the [Visualizer Guide](docs/visualizer-guide.md) for setup and usage instructions.
*   **VisionFlow System**: This ontology module is a component of the broader VisionFlow Knowledge Management System, which provides advanced visualization and knowledge exploration capabilities.

![VisionFlow Knowledge Management System](https://raw.githubusercontent.com/DreamLab-AI/VisionFlow/main/visionflow.gif)

To use these visualizers, the ontology must first be converted from its native OWL Functional Syntax (.ofn) to the Turtle (.ttl) format.

## 📁 Project Structure

```
Metaverse-Ontology/
├── README.md                          # This file
│
├── docs/                              # 📚 Documentation
│   ├── guides/
│   │   ├── QUICKSTART.md             # 5-minute setup guide
│   │   └── USER_GUIDE.md             # Guide to using the ontology
│   ├── reference/
│   │   ├── TEMPLATE.md               # Template for new concepts
│   │   └── URIMapping.md             # Rules for converting Wikilinks to IRIs
│   └── visualizer-guide.md           # Guide for the OWL TTL Web Visualizer
│
├── OntologyDefinition.md             # 🎯 Core ontology header & base classes
├── PropertySchema.md                  # 🔗 All defined properties
├── ETSIDomainClassification.md       # 🏛️ ETSI domain taxonomy
│
├── Avatar.md                          # 📘 Example: VirtualAgent class
├── DigitalTwin.md                     # 📗 Example: HybridObject class
│
├── *.md                              # Over 280 other concept files
│
├── logseq-owl-extractor/             # 🦀 Rust tool for OWL extraction
│
└── convert_owl_to_ttl.py             # 🐍 Python script for TTL conversion
```

## 🚀 Quick Start

### Prerequisites
*   **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/).
*   **Python 3**: For the conversion script.
*   **ROBOT (optional but recommended)**: A command-line tool for ontology tasks. Download it from [http://robot.obolibrary.org/](http://robot.obolibrary.org/).

### 1. Build the Extractor

```bash
cd logseq-owl-extractor
cargo build --release
cd ..
```

### 2. Extract the Ontology (.ofn)

This command parses all markdown files and generates a single ontology file in OWL Functional Syntax.

```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --validate
```

### 3. Convert to Turtle (.ttl) for Visualization

Use a tool like ROBOT to convert the `.ofn` output to `.ttl`.

```bash
robot convert --input metaverse-ontology.ofn --output metaverse-ontology.ttl
```
*Note: The included `convert_owl_to_ttl.py` is a basic script; ROBOT is recommended for more robust conversions.*

### 4. Visualize the Ontology

Follow the instructions in the [**Visualizer Guide**](docs/visualizer-guide.md) to load your `metaverse-ontology.ttl` file into the OWL TTL Web Visualizer.

## 🎯 Design Philosophy

### Orthogonal Classification

The ontology uses two independent dimensions to classify every concept:

| Physicality | Role | → Inferred Class | Example |
|---|---|---|---|
| Physical | Object | **PhysicalObject** | VR Headset |
| Virtual | Agent | **VirtualAgent** | Avatar |
| Hybrid | Object | **HybridObject** | Digital Twin |
| *(...and 6 other combinations)* | | |

This design allows for powerful automated classification and ensures a clean separation of concerns.

### Logseq-Native Format

Each concept is defined in a human-readable Logseq outline, which is then parsed by the extractor.

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
		- has-part:: [[Visual Mesh]], [[Animation Rig]]
	- #### OWL Axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:Avatar))
		  SubClassOf(mv:Avatar mv:VirtualEntity)
		  SubClassOf(mv:Avatar mv:Agent)
		  ```
- ## About Avatars
	- Human-readable description and examples...
```

**Benefits of this approach**:
*   **Human-Readable**: Easy to browse and edit in Logseq.
*   **Machine-Readable**: The Rust tool reliably extracts formal axioms.
*   **Linked Data**: `[[Wikilinks]]` create a connected knowledge graph within Logseq.
*   **Documented**: Formal definitions live alongside rich, human-readable context.

## 🔧 Technology Stack

| Component | Technology | Purpose |
|---|---|---|
| **Ontology Language** | OWL 2 DL | Formal knowledge representation |
| **Source Format** | Logseq Markdown | Human-readable editing and documentation |
| **Extraction Tool** | Custom Rust Tool | Converts Logseq markdown to OWL |
| **Parser/Validator** | horned-owl (Rust) | OWL parsing and syntax validation |
| **Visualization** | OWL TTL Web Visualizer | Interactive 3D graph exploration |
| **Version Control** | Git | Tracks changes to the ontology source |

## 🤝 Contributing

Contributions are highly encouraged! To add or improve a concept:

1.  **Read the guidelines**: See the full [CONTRIBUTING.md](CONTRIBUTING.md) file.
2.  **Use the template**: Copy [docs/reference/TEMPLATE.md](docs/reference/TEMPLATE.md).
3.  **Classify correctly**: Apply the Physicality × Role dimensions.
4.  **Add the tag**: Ensure `metaverseOntology:: true` is present.
5.  **Validate your changes**: Run the extractor tool to check for syntax errors.
6.  **Submit a Pull Request**.

## 📄 License

This project is licensed under the Mozilla Public License 2.0. See the `LICENSE` file for details.