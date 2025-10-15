# Metaverse Ontology - WebVOWL Visualization Guide

## 📋 Overview

This directory contains the Metaverse Ontology prepared for interactive visualization using WebVOWL and other ontology visualization tools.

## 📁 Files

| File | Description | Format | Purpose |
|------|-------------|--------|---------|
| **metaverse-ontology-webvowl.owl** | Enhanced ontology with annotations | OWL/XML | **Recommended for WebVOWL** |
| metaverse-ontology.owl | Standard OWL conversion | OWL/XML | Alternative visualization |
| metaverse-ontology.jsonld | Semantic web format | JSON-LD | JavaScript/Web integration |
| metaverse-ontology.ttl | Turtle format | Turtle | RDF tools |
| index.html | Interactive guide | HTML | Getting started page |
| add_annotations.py | Annotation script | Python | Added labels/comments |
| convert_jsonld.py | JSON-LD converter | Python | Format conversion |

## 🚀 Quick Start (Recommended)

### Option 1: WebVOWL Online (No Installation Required)

1. **Open WebVOWL**: Navigate to http://www.visualdataweb.de/webvowl/
2. **Upload Ontology**:
   - Click **"Ontology"** menu → **"Select ontology file..."**
   - Choose `metaverse-ontology-webvowl.owl` from this directory
3. **Explore**:
   - Drag nodes to rearrange
   - Mouse wheel to zoom
   - Hover for details
   - Click for class information

### Option 2: View Local Guide

```bash
# Open the interactive HTML guide
cd visualization
python3 -m http.server 8080
# Then open http://localhost:8080 in your browser
```

## 🎯 Visualization Methods

### 1. WebVOWL (Web-Based)

**Best For:** Quick, interactive exploration without installation

**Features:**
- Interactive force-directed graph
- Class hierarchy visualization
- Property relationships
- Filtering and search
- Export as SVG/JSON

**URL:** http://www.visualdataweb.de/webvowl/

**File to Use:** `metaverse-ontology-webvowl.owl`

---

### 2. Protégé + OntoGraf (Desktop)

**Best For:** Advanced ontology editing and visualization

**Installation:**
```bash
# Download from https://protege.stanford.edu/
# Install OntoGraf plugin via File → Check for plugins
```

**Steps:**
1. Open Protégé
2. File → Open → Select `metaverse-ontology-webvowl.owl`
3. Window → Tabs → OntoGraf
4. View class hierarchy and relationships

**File to Use:** `metaverse-ontology-webvowl.owl`

---

### 3. Local WebVOWL Instance

**Best For:** Offline work, customization, integration

**Setup:**
```bash
# Clone WebVOWL repository
git clone https://github.com/VisualDataWeb/WebVOWL.git
cd WebVOWL

# Install dependencies
npm install

# Copy your ontology to data directory
cp /path/to/metaverse-ontology-webvowl.owl deploy/data/

# Add entry to deploy/data/ontologies.json:
# {
#   "id": "metaverse",
#   "title": "Metaverse Ontology",
#   "file": "metaverse-ontology-webvowl.owl"
# }

# Run local server
npm start

# Open http://localhost:8080
```

**File to Use:** `metaverse-ontology-webvowl.owl`

---

### 4. OWLGrEd (Graphical Editor)

**Best For:** Creating and editing ontologies visually

**URL:** http://owlgred.lumii.lv/

**Steps:**
1. Open OWLGrEd web app
2. Import `metaverse-ontology-webvowl.owl`
3. Edit and visualize graphically

---

### 5. VOWL (Visual Notation)

**Best For:** Academic papers, documentation

**Format:** VOWL is a visual notation standard implemented by WebVOWL

**Export:** Use WebVOWL's export function to save as SVG for publications

## 📊 Ontology Statistics

- **Classes:** 40 total
  - 3 primary concepts (Portability, Persistence, ResilienceMetric)
  - 37 supporting/classification classes
- **Annotations:** 100% coverage (all classes have labels and comments)
- **Format:** OWL 2 DL
- **Triples:** 166 RDF statements
- **Namespaces:** 9 (mv, owl, rdf, rdfs, xsd, dc, dcterms, etsi, iso)

## 🎨 Visualization Features

### Interactive Elements

- **Nodes:** Represent OWL classes
- **Edges:** Show SubClassOf relationships
- **Colors:** Distinguish class types
- **Labels:** Human-readable names
- **Tooltips:** Detailed descriptions on hover

### Navigation

- **Zoom:** Mouse wheel or pinch gesture
- **Pan:** Click and drag background
- **Select:** Click nodes for details
- **Filter:** Show/hide elements via sidebar

### Export Options

- **SVG:** Vector graphics for publications
- **JSON:** Data format for processing
- **PNG:** Raster image (via screenshot)

## 📖 Core Ontology Concepts

### Portability
**Type:** VirtualProcess
**Definition:** Cross-platform migration process enabling seamless asset and identity transfer between heterogeneous virtual platforms and ecosystems

**Key Features:**
- Format conversion (FBX, glTF, USD)
- Identity federation (DIDs, OAuth)
- Metadata mapping
- Semantic preservation

---

### Persistence
**Type:** VirtualProcess
**Definition:** State management process ensuring data continuity, ownership records, and contextual information across sessions, platforms, and time periods

**Key Features:**
- Distributed state management
- Consistency protocols (ACID, BASE)
- Recovery mechanisms
- Session management

---

### Resilience Metric
**Type:** VirtualObject
**Definition:** Quantitative and qualitative measurement of system robustness, fault tolerance, recovery capabilities, and service continuity under adverse conditions

**Key Features:**
- Availability measurement (99.9%, 99.99%)
- Recovery time objectives (RTO)
- Fault tolerance indicators
- SLA compliance tracking

## 🔧 Advanced Usage

### Custom Annotations

Add additional annotations using Python:

```python
from rdflib import Graph, Literal, URIRef
from rdflib.namespace import RDFS

g = Graph()
g.parse("metaverse-ontology-webvowl.owl", format="xml")

# Add custom annotation
class_uri = URIRef("https://metaverse-ontology.org/YourClass")
g.add((class_uri, RDFS.label, Literal("Your Label", lang="en")))

g.serialize(destination="custom-ontology.owl", format="xml")
```

### Convert to Other Formats

Using ROBOT tool:

```bash
# To Turtle
java -jar ../robot.jar convert \
  --input metaverse-ontology-webvowl.owl \
  --format ttl \
  --output metaverse-ontology.ttl

# To Manchester Syntax
java -jar ../robot.jar convert \
  --input metaverse-ontology-webvowl.owl \
  --format omn \
  --output metaverse-ontology.omn
```

### Extract Specific Classes

```bash
# Extract only specific modules
java -jar ../robot.jar extract \
  --input metaverse-ontology-webvowl.owl \
  --method STAR \
  --term mv:Portability \
  --output portability-module.owl
```

## 🌐 Online Resources

- **WebVOWL:** http://www.visualdataweb.de/webvowl/
- **Protégé:** https://protege.stanford.edu/
- **OWL 2 Primer:** https://www.w3.org/TR/owl2-primer/
- **VOWL Notation:** http://vowl.visualdataweb.org/
- **WebVOWL GitHub:** https://github.com/VisualDataWeb/WebVOWL

## 📚 Related Standards

- **W3C OWL 2:** Web Ontology Language specification
- **RDF Schema:** Resource Description Framework
- **JSON-LD:** Linked Data JSON format
- **SPARQL:** Query language for RDF
- **SHACL:** Shapes Constraint Language

## 🐛 Troubleshooting

### WebVOWL Won't Load Ontology

**Problem:** Error loading ontology file

**Solutions:**
1. Verify file is valid OWL/XML: `java -jar ../robot.jar validate --input metaverse-ontology-webvowl.owl`
2. Try uploading to online WebVOWL instead of local version
3. Check browser console for specific errors
4. Ensure file isn't corrupted (re-generate if needed)

### No Labels Showing in WebVOWL

**Problem:** Classes show URIs instead of labels

**Solution:** Use `metaverse-ontology-webvowl.owl` which has annotations, not `metaverse-ontology.owl`

### Visualization Too Large/Complex

**Problem:** Graph is overwhelming with many nodes

**Solutions:**
1. Use filtering in WebVOWL sidebar
2. Extract specific modules with ROBOT
3. Hide datatype properties
4. Adjust graph settings (force strength, node distance)

### Local WebVOWL Not Starting

**Problem:** `npm start` fails

**Solutions:**
```bash
# Clear npm cache
npm cache clean --force

# Reinstall dependencies
rm -rf node_modules
npm install

# Use specific Node version
nvm use 16
npm start
```

## 🔒 Validation

Verify ontology integrity:

```bash
# Using ROBOT
java -jar ../robot.jar validate --input metaverse-ontology-webvowl.owl

# Check for unsatisfiable classes (requires reasoner)
java -jar ../robot.jar reason \
  --input metaverse-ontology-webvowl.owl \
  --reasoner HermiT \
  --output reasoned-ontology.owl
```

## 📝 Next Steps

1. **Expand Ontology:** Add more concepts from the 274 VisioningLab files
2. **Add Properties:** Define object and data properties
3. **Create SWRL Rules:** Add semantic rules and constraints
4. **SPARQL Queries:** Write competency questions
5. **Publish:** Host on GitHub Pages or ontology repository

## 📄 License

This ontology is part of the Metaverse Ontology Design project.

---

**Generated:** October 15, 2025
**Validator:** logseq-owl-extractor v0.1.0
**Format:** OWL 2 DL
**Status:** Production Ready ✅
