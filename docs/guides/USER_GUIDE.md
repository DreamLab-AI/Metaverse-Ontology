# Metaverse Ontology User Guide

A comprehensive guide to using, querying, and integrating the Metaverse Ontology after migration.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Exploring the Ontology](#exploring-the-ontology)
4. [Querying the Ontology](#querying-the-ontology)
5. [Integration Examples](#integration-examples)
6. [Understanding the Structure](#understanding-the-structure)
7. [Use Cases](#use-cases)
8. [Troubleshooting](#troubleshooting)

---

## Introduction

### What This Guide Covers

This guide is designed for **post-migration usage** of the Metaverse Ontology. It covers:

- How to explore and visualize the ontology
- Querying techniques using SPARQL
- Integration into applications using Python, JavaScript, and Java
- Understanding the ontology's orthogonal classification system
- Practical use cases and examples

### Who This Guide Is For

- **Developers**: Integrating the ontology into metaverse applications
- **Researchers**: Analyzing metaverse concepts and relationships
- **Data Scientists**: Performing semantic queries and inference
- **Ontology Engineers**: Extending or customizing the ontology
- **Students**: Learning about semantic technologies and metaverse standards

### Prerequisites

- Basic understanding of RDF/OWL concepts
- Familiarity with SPARQL query language (helpful but not required)
- Programming experience in Python, JavaScript, or Java (for integration examples)

---

## Getting Started

### Quick Setup

#### 1. Build the OWL Extractor

First, compile the Rust-based extraction tool:

```bash
cd /home/john/githubs/OntologyDesign/logseq-owl-extractor
cargo build --release
```

#### 2. Generate the OWL Ontology

Extract OWL from the Logseq markdown files:

```bash
cd /home/john/githubs/OntologyDesign
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --validate
```

This creates `metaverse-ontology.ofn` in OWL Functional Syntax.

#### 3. Convert to Your Preferred Format

The ontology can be converted to various formats:

```bash
# Convert to RDF/XML
robot convert --input metaverse-ontology.ofn \
              --output metaverse-ontology.owl

# Convert to Turtle (TTL)
robot convert --input metaverse-ontology.ofn \
              --output metaverse-ontology.ttl

# Convert to JSON-LD
robot convert --input metaverse-ontology.ofn \
              --output metaverse-ontology.jsonld
```

**Note**: Install [ROBOT](http://robot.obolibrary.org/) ontology toolkit if not already available:
```bash
pip install ontospy
# or download ROBOT from http://robot.obolibrary.org/
```

#### 4. Verify Installation

Check the generated ontology:

```bash
head -n 30 metaverse-ontology.ofn
```

You should see prefix declarations, the ontology IRI, and axiom declarations.

---

## Exploring the Ontology

### Using WebVOWL Visualization

WebVOWL provides an interactive graph-based visualization of the ontology.

#### Generate WebVOWL-Compatible OWL

If you have an OWL file (e.g., `metaverse-ontology.owl`), you can visualize it using WebVOWL:

1. **Visit WebVOWL Online**: [http://vowl.visualdataweb.org/webvowl.html](http://vowl.visualdataweb.org/webvowl.html)

2. **Upload Your Ontology**:
   - Click "Ontology" menu
   - Select "Upload Ontology"
   - Choose `metaverse-ontology.owl`

3. **Explore the Graph**:
   - **Zoom/Pan**: Use mouse wheel to zoom, click-drag to pan
   - **Node Details**: Click on any class or property to see details
   - **Filters**: Use the filter panel to focus on specific domains or classes
   - **Export**: Save the visualization as SVG or PNG

#### Local WebVOWL Setup

For offline visualization:

```bash
# Clone WebVOWL
git clone https://github.com/VisualDataWeb/WebVOWL.git
cd WebVOWL

# Serve locally
python3 -m http.server 8000

# Open in browser
# Navigate to http://localhost:8000
# Upload metaverse-ontology.owl
```

### Opening in Protégé

Protégé is the industry-standard OWL ontology editor.

#### Install Protégé

Download from [https://protege.stanford.edu/](https://protege.stanford.edu/)

#### Open the Ontology

```bash
protege /home/john/githubs/OntologyDesign/metaverse-ontology.ofn
```

#### Key Protégé Views

1. **Classes Tab**:
   - View the class hierarchy
   - See `PhysicalEntity`, `VirtualEntity`, `HybridEntity`
   - Explore `Agent`, `Object`, `Environment` roles
   - Examine inferred classes like `VirtualAgent`, `HybridObject`

2. **Object Properties Tab**:
   - Review relationships: `hasPart`, `dependsOn`, `enables`, `bindsTo`
   - Check property characteristics (transitive, symmetric, etc.)

3. **Data Properties Tab**:
   - See `hasLatency`, `hasResolution`, `hasCapacity`

4. **Individuals Tab**:
   - Browse instances (if any are defined)

5. **DL Query Tab**:
   - Run description logic queries
   - Example: Find all virtual agents
     ```
     VirtualAgent
     ```

### Browsing with whelk-rs

whelk-rs is a fast OWL reasoner written in Rust.

#### Install whelk-rs

```bash
cargo install whelk
```

#### Classify the Ontology

```bash
whelk classify /home/john/githubs/OntologyDesign/metaverse-ontology.ofn
```

This performs reasoning and infers implicit class memberships.

#### Query with whelk-rs

```bash
# Find all subclasses of VirtualAgent
whelk query "SubClassOf(?x mv:VirtualAgent)" \
  /home/john/githubs/OntologyDesign/metaverse-ontology.ofn

# Find all classes with hasPart relationships
whelk query "ObjectProperty(?x mv:hasPart ?y)" \
  /home/john/githubs/OntologyDesign/metaverse-ontology.ofn
```

---

## Querying the Ontology

### Setting Up a SPARQL Endpoint

To query with SPARQL, you need a triplestore. We'll use Apache Jena Fuseki.

#### Install Jena Fuseki

```bash
# Download Fuseki
wget https://dlcdn.apache.org/jena/binaries/apache-jena-fuseki-4.10.0.tar.gz
tar -xzf apache-jena-fuseki-4.10.0.tar.gz
cd apache-jena-fuseki-4.10.0
```

#### Load the Ontology

```bash
# Start Fuseki server
./fuseki-server --port=3030 &

# Create dataset
curl -X POST http://localhost:3030/$/datasets \
  -d "dbName=metaverse&dbType=mem" \
  --header "Content-Type: application/x-www-form-urlencoded"

# Upload ontology (ensure it's in RDF/XML or Turtle)
curl -X POST http://localhost:3030/metaverse/data \
  --data-binary @/home/john/githubs/OntologyDesign/metaverse-ontology.ttl \
  --header "Content-Type: text/turtle"
```

Now the SPARQL endpoint is available at `http://localhost:3030/metaverse/query`.

### Basic SPARQL Queries

#### Query 1: List All Classes

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

SELECT ?class ?label
WHERE {
  ?class a owl:Class .
  OPTIONAL { ?class rdfs:label ?label }
}
ORDER BY ?class
```

**Expected Results**: All ontology classes including `VirtualAgent`, `PhysicalObject`, `HybridAgent`, etc.

#### Query 2: Find All Virtual Agents

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?agent ?label ?definition
WHERE {
  ?agent rdfs:subClassOf* mv:VirtualAgent .
  OPTIONAL { ?agent rdfs:label ?label }
  OPTIONAL { ?agent mv:definition ?definition }
}
```

**Expected Results**: `Avatar`, `AI Agent`, `Virtual Assistant`, and other virtual agent subclasses.

#### Query 3: Get Concepts by ETSI Domain

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept ?domain
WHERE {
  ?concept mv:belongsToDomain ?domain .
  FILTER(CONTAINS(STR(?domain), "InteractionDomain"))
}
```

**Expected Results**: All concepts belonging to the `InteractionDomain` (e.g., `Avatar`, `Gesture`, `Social Presence`).

### Common Query Patterns

#### Pattern 1: Find All Parts of a Concept

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept ?part
WHERE {
  mv:Avatar mv:hasPart ?part .
}
```

**Use Case**: Discover what components make up an Avatar (Visual Mesh, Animation Rig).

#### Pattern 2: Find Dependencies

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept ?dependency
WHERE {
  ?concept mv:dependsOn ?dependency .
}
```

**Use Case**: Identify what infrastructure or technologies a concept requires.

#### Pattern 3: Find Enabled Capabilities

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept ?capability
WHERE {
  ?concept mv:enables ?capability .
}
```

**Use Case**: Understand what functionalities or experiences a technology enables.

#### Pattern 4: Physicality-Based Filtering

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept
WHERE {
  ?concept mv:physicality "VirtualEntity" .
}
```

**Use Case**: Filter concepts by physicality dimension (Physical, Virtual, Hybrid).

#### Pattern 5: Role-Based Filtering

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>

SELECT ?concept
WHERE {
  ?concept mv:role "Agent" .
}
```

**Use Case**: Find all agent-type entities regardless of physicality.

### Using Reasoner for Inference

Enable reasoning to infer implicit relationships:

```sparql
PREFIX mv: <http://www.metaverse-ontology.org/mv#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

# With reasoning enabled, this query will also return
# inferred instances of VirtualAgent due to intersection axioms
SELECT ?agent
WHERE {
  ?agent a mv:VirtualAgent .
}
```

**Note**: Ensure your triplestore has reasoning enabled. For Fuseki, use a TDB dataset with OWL inference.

---

## Integration Examples

### Python with rdflib

Install rdflib:

```bash
pip install rdflib
```

#### Load and Query the Ontology

```python
from rdflib import Graph, Namespace, RDF, RDFS

# Load ontology
g = Graph()
g.parse("/home/john/githubs/OntologyDesign/metaverse-ontology.ttl", format="turtle")

# Define namespace
MV = Namespace("http://www.metaverse-ontology.org/mv#")
g.bind("mv", MV)

# Query: Find all Virtual Agents
query = """
    PREFIX mv: <http://www.metaverse-ontology.org/mv#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

    SELECT ?agent ?label
    WHERE {
        ?agent rdfs:subClassOf* mv:VirtualAgent .
        OPTIONAL { ?agent rdfs:label ?label }
    }
"""

results = g.query(query)

print("Virtual Agents:")
for row in results:
    print(f"  - {row.agent}: {row.label}")
```

#### Traverse Relationships

```python
# Find all parts of Avatar
avatar = MV.Avatar
parts = g.objects(subject=avatar, predicate=MV.hasPart)

print("Avatar parts:")
for part in parts:
    print(f"  - {part}")
```

#### Check Class Membership

```python
# Check if Avatar is a VirtualAgent
is_virtual_agent = (MV.Avatar, RDFS.subClassOf, MV.VirtualAgent) in g
print(f"Avatar is a VirtualAgent: {is_virtual_agent}")
```

#### Export Subgraph

```python
# Extract all concepts in InteractionDomain
interaction_query = """
    PREFIX mv: <http://www.metaverse-ontology.org/mv#>

    CONSTRUCT {
        ?concept ?p ?o
    }
    WHERE {
        ?concept mv:belongsToDomain mv:InteractionDomain .
        ?concept ?p ?o .
    }
"""

interaction_graph = g.query(interaction_query).graph
interaction_graph.serialize("interaction_domain.ttl", format="turtle")
print("Exported InteractionDomain concepts to interaction_domain.ttl")
```

### JavaScript with rdf.js

Install dependencies:

```bash
npm install rdf-ext rdf-parser-n3 rdf-fetch
```

#### Load and Query

```javascript
const rdf = require('rdf-ext');
const ParserN3 = require('rdf-parser-n3');
const fs = require('fs');

// Load ontology
const parserN3 = new ParserN3();
const stream = fs.createReadStream('/home/john/githubs/OntologyDesign/metaverse-ontology.ttl');
const quadStream = parserN3.import(stream);

const dataset = rdf.dataset();

quadStream.on('data', (quad) => {
  dataset.add(quad);
});

quadStream.on('end', () => {
  console.log(`Loaded ${dataset.size} triples`);

  // Query: Find all VirtualAgent subclasses
  const MV = rdf.namespace('http://www.metaverse-ontology.org/mv#');
  const RDFS = rdf.namespace('http://www.w3.org/2000/01/rdf-schema#');

  const virtualAgents = dataset.match(null, RDFS.subClassOf, MV.VirtualAgent);

  console.log('Virtual Agents:');
  for (const quad of virtualAgents) {
    console.log(`  - ${quad.subject.value}`);
  }
});
```

#### Using SPARQL.js

```bash
npm install sparqljs
```

```javascript
const fs = require('fs');
const { Parser } = require('sparqljs');

// Parse SPARQL query
const parser = new Parser();
const query = `
  PREFIX mv: <http://www.metaverse-ontology.org/mv#>
  SELECT ?agent WHERE {
    ?agent rdfs:subClassOf mv:VirtualAgent .
  }
`;

const parsedQuery = parser.parse(query);
console.log(JSON.stringify(parsedQuery, null, 2));

// Execute against a SPARQL endpoint (requires fetch)
const fetch = require('node-fetch');

async function executeSparql() {
  const endpoint = 'http://localhost:3030/metaverse/query';
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: { 'Content-Type': 'application/sparql-query' },
    body: query
  });

  const results = await response.json();
  console.log('Results:', results);
}

executeSparql();
```

### Java with Apache Jena

Add Maven dependency:

```xml
<dependency>
    <groupId>org.apache.jena</groupId>
    <artifactId>apache-jena-libs</artifactId>
    <version>4.10.0</version>
</dependency>
```

#### Load and Query

```java
import org.apache.jena.rdf.model.*;
import org.apache.jena.query.*;

public class MetaverseOntologyExample {
    public static void main(String[] args) {
        // Load ontology
        Model model = ModelFactory.createDefaultModel();
        model.read("/home/john/githubs/OntologyDesign/metaverse-ontology.ttl", "TURTLE");

        System.out.println("Loaded " + model.size() + " triples");

        // Define namespace
        String mvNS = "http://www.metaverse-ontology.org/mv#";

        // SPARQL Query: Find all Virtual Agents
        String queryString =
            "PREFIX mv: <" + mvNS + "> " +
            "PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> " +
            "SELECT ?agent ?label WHERE { " +
            "  ?agent rdfs:subClassOf* mv:VirtualAgent . " +
            "  OPTIONAL { ?agent rdfs:label ?label } " +
            "}";

        Query query = QueryFactory.create(queryString);
        QueryExecution qexec = QueryExecutionFactory.create(query, model);

        try {
            ResultSet results = qexec.execSelect();
            System.out.println("Virtual Agents:");
            while (results.hasNext()) {
                QuerySolution soln = results.nextSolution();
                Resource agent = soln.getResource("agent");
                Literal label = soln.getLiteral("label");
                System.out.println("  - " + agent + ": " + label);
            }
        } finally {
            qexec.close();
        }
    }
}
```

#### Reasoning with Jena

```java
import org.apache.jena.rdf.model.*;
import org.apache.jena.reasoner.*;
import org.apache.jena.reasoner.rulesys.*;

public class MetaverseReasoning {
    public static void main(String[] args) {
        // Load ontology
        Model schema = ModelFactory.createDefaultModel();
        schema.read("/home/john/githubs/OntologyDesign/metaverse-ontology.ttl", "TURTLE");

        // Create reasoner
        Reasoner reasoner = ReasonerRegistry.getOWLReasoner();
        reasoner = reasoner.bindSchema(schema);

        // Create inference model
        InfModel infModel = ModelFactory.createInfModel(reasoner, schema);

        // Validate consistency
        ValidityReport validity = infModel.validate();
        if (validity.isValid()) {
            System.out.println("Ontology is consistent");
        } else {
            System.out.println("Conflicts:");
            for (ValidityReport.Report report : validity.getReports()) {
                System.out.println(" - " + report);
            }
        }

        // Query inferred classes
        String mvNS = "http://www.metaverse-ontology.org/mv#";
        Resource virtualAgent = infModel.getResource(mvNS + "VirtualAgent");

        StmtIterator iter = infModel.listStatements(null, RDFS.subClassOf, virtualAgent);
        System.out.println("Inferred Virtual Agents:");
        while (iter.hasNext()) {
            Statement stmt = iter.nextStatement();
            System.out.println("  - " + stmt.getSubject());
        }
    }
}
```

---

## Understanding the Structure

### Orthogonal Classification: Physicality × Role

The ontology uses a **two-dimensional classification system** that enables powerful inference:

#### Physicality Dimension

| Class | Description | Examples |
|-------|-------------|----------|
| `PhysicalEntity` | Exists in the physical world | VR Headset, Human, Haptic Glove |
| `VirtualEntity` | Exists only in digital form | Avatar, 3D Model, Virtual World |
| `HybridEntity` | Bridges physical and virtual | Digital Twin, AR Overlay, Cyborg |

#### Role Dimension

| Class | Description | Examples |
|-------|-------------|----------|
| `Agent` | Acts autonomously | Human, Avatar, AI Agent |
| `Object` | Passive entity | 3D Asset, VR Headset, Digital Twin |
| `Environment` | Spatial context | Virtual World, Mixed Reality Space |

#### Inferred Intersection Classes

The reasoner **automatically infers** 9 intersection classes:

| Physicality | Role | Inferred Class | Example |
|-------------|------|----------------|---------|
| Physical | Agent | `PhysicalAgent` | Human |
| Virtual | Agent | `VirtualAgent` | Avatar |
| Hybrid | Agent | `HybridAgent` | Cyborg |
| Physical | Object | `PhysicalObject` | VR Headset |
| Virtual | Object | `VirtualObject` | 3D Model |
| Hybrid | Object | `HybridObject` | Digital Twin |
| Physical | Environment | `PhysicalEnvironment` | Physical Room |
| Virtual | Environment | `VirtualEnvironment` | Virtual World |
| Hybrid | Environment | `HybridEnvironment` | AR Space |

**How It Works**:

```owl
# Define Avatar as Virtual + Agent
SubClassOf(mv:Avatar mv:VirtualEntity)
SubClassOf(mv:Avatar mv:Agent)

# Reasoner infers:
SubClassOf(mv:Avatar mv:VirtualAgent)
```

### ETSI Domains

Concepts are classified into **7 ETSI metaverse domains**:

| Domain | Description | Example Concepts |
|--------|-------------|------------------|
| `UserDomain` | End-user aspects | Human, User Profile |
| `InteractionDomain` | User interaction | Avatar, Gesture, Voice |
| `ApplicationDomain` | Metaverse applications | Virtual World, Game |
| `ComputeDomain` | Computational resources | Edge Server, Cloud |
| `NetworkDomain` | Connectivity | 5G, WiFi, CDN |
| `InfrastructureDomain` | Physical infrastructure | Data Center, VR Headset |
| `ManagementDomain` | Operations & governance | Identity, Policy |

### Relationships and Properties

#### Object Properties

| Property | Type | Description | Example |
|----------|------|-------------|---------|
| `hasPart` | Transitive, Irreflexive, Asymmetric | Mereological composition | Avatar hasPart Visual Mesh |
| `isPartOf` | Transitive | Inverse of hasPart | Visual Mesh isPartOf Avatar |
| `dependsOn` | Transitive | Functional dependency | Avatar dependsOn 3D Rendering Engine |
| `requires` | Sub-property of dependsOn | Strong dependency | AR Overlay requires Camera |
| `enables` | General | Capability provision | Avatar enables Social Presence |
| `bindsTo` | Symmetric, Irreflexive, Functional | Physical-virtual binding | Digital Twin bindsTo Physical Object |
| `represents` | General | Representation relationship | Avatar represents Human |

#### Data Properties

| Property | Range | Description | Example |
|----------|-------|-------------|---------|
| `hasLatency` | xsd:float | Network latency in ms | 5G hasLatency 10.5 |
| `hasResolution` | xsd:string | Display resolution | VR Headset hasResolution "4K" |
| `hasCapacity` | xsd:integer | Storage/compute capacity | Server hasCapacity 1024 |

#### Annotation Properties

| Property | Purpose | Example |
|----------|---------|---------|
| `definition` | Human-readable definition | "Digital representation of a person" |
| `source` | Standards reference | "ACM + Web3D HAnim" |
| `maturity` | Development stage | "mature", "emerging" |
| `termId` | Unique identifier | "20067" |

---

## Use Cases

### Use Case 1: Building a Metaverse Application

**Scenario**: You're building a social VR platform and need to model users, avatars, and virtual spaces.

**Approach**:

1. **Identify Required Concepts**:
   ```sparql
   # Find all relevant concepts in InteractionDomain
   PREFIX mv: <http://www.metaverse-ontology.org/mv#>

   SELECT ?concept ?definition
   WHERE {
     ?concept mv:belongsToDomain mv:InteractionDomain ;
              mv:definition ?definition .
   }
   ```

2. **Understand Dependencies**:
   ```sparql
   # What does Avatar require?
   SELECT ?dependency
   WHERE {
     mv:Avatar mv:requires ?dependency .
   }
   ```

3. **Model Your Data**:
   ```python
   from rdflib import Graph, Namespace, Literal, URIRef

   # Create application graph
   app = Graph()
   MV = Namespace("http://www.metaverse-ontology.org/mv#")
   APP = Namespace("http://myapp.example.com/")

   # Define a user
   user1 = URIRef(APP.user_12345)
   app.add((user1, RDF.type, MV.Human))
   app.add((user1, MV.hasName, Literal("Alice")))

   # Define their avatar
   avatar1 = URIRef(APP.avatar_12345)
   app.add((avatar1, RDF.type, MV.Avatar))
   app.add((avatar1, MV.represents, user1))

   # Save
   app.serialize("my_app_data.ttl", format="turtle")
   ```

### Use Case 2: Semantic Validation

**Scenario**: Validate that your metaverse architecture conforms to standards.

**Approach**:

1. **Load Ontology and Your Data**:
   ```python
   from rdflib import Graph
   from rdflib.plugins.stores.sparqlstore import SPARQLUpdateStore

   # Load ontology
   ontology = Graph()
   ontology.parse("metaverse-ontology.ttl", format="turtle")

   # Load your architecture
   arch = Graph()
   arch.parse("my_architecture.ttl", format="turtle")

   # Merge
   combined = ontology + arch
   ```

2. **Run Validation Queries**:
   ```sparql
   # Check: All Avatars must represent exactly one Agent
   PREFIX mv: <http://www.metaverse-ontology.org/mv#>

   SELECT ?avatar (COUNT(?agent) AS ?count)
   WHERE {
     ?avatar a mv:Avatar ;
             mv:represents ?agent .
   }
   GROUP BY ?avatar
   HAVING (COUNT(?agent) != 1)
   ```

3. **Report Violations**:
   ```python
   results = combined.query(validation_query)

   if len(results) > 0:
       print("Validation failed:")
       for row in results:
           print(f"  Avatar {row.avatar} represents {row.count} agents (expected 1)")
   else:
       print("Validation passed!")
   ```

### Use Case 3: Research and Analysis

**Scenario**: Analyze the metaverse technology landscape and identify gaps.

**Approach**:

1. **Count Concepts by Domain**:
   ```sparql
   PREFIX mv: <http://www.metaverse-ontology.org/mv#>

   SELECT ?domain (COUNT(?concept) AS ?count)
   WHERE {
     ?concept mv:belongsToDomain ?domain .
   }
   GROUP BY ?domain
   ORDER BY DESC(?count)
   ```

2. **Identify Mature vs. Emerging Technologies**:
   ```sparql
   SELECT ?maturity (COUNT(?concept) AS ?count)
   WHERE {
     ?concept mv:maturity ?maturity .
   }
   GROUP BY ?maturity
   ```

3. **Map Technology Dependencies**:
   ```python
   import networkx as nx
   from rdflib import Graph

   g = Graph()
   g.parse("metaverse-ontology.ttl", format="turtle")

   # Build dependency graph
   dep_graph = nx.DiGraph()

   query = """
       PREFIX mv: <http://www.metaverse-ontology.org/mv#>
       SELECT ?concept ?dependency
       WHERE {
         ?concept mv:dependsOn ?dependency .
       }
   """

   for row in g.query(query):
       dep_graph.add_edge(str(row.concept), str(row.dependency))

   # Find central technologies
   centrality = nx.betweenness_centrality(dep_graph)
   print("Most critical technologies:")
   for tech, score in sorted(centrality.items(), key=lambda x: x[1], reverse=True)[:10]:
       print(f"  {tech}: {score:.3f}")
   ```

### Use Case 4: Extending the Ontology

**Scenario**: Add your own domain-specific concepts.

**Approach**:

1. **Create Extension File**:
   ```turtle
   @prefix mv: <http://www.metaverse-ontology.org/mv#> .
   @prefix myext: <http://mycompany.com/metaverse-extension#> .
   @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
   @prefix owl: <http://www.w3.org/2002/07/owl#> .

   # Define your custom class
   myext:HolographicDisplay a owl:Class ;
       rdfs:subClassOf mv:PhysicalObject ;
       mv:definition "A display that projects 3D holograms" ;
       mv:belongsToDomain mv:InfrastructureDomain ;
       mv:maturity "emerging" .

   # Add relationships
   myext:HolographicDisplay mv:enables mv:AROverlay .
   ```

2. **Load Combined Ontology**:
   ```python
   from rdflib import Graph

   g = Graph()
   g.parse("metaverse-ontology.ttl", format="turtle")
   g.parse("my_extension.ttl", format="turtle")

   # Now query including your extensions
   query = """
       PREFIX mv: <http://www.metaverse-ontology.org/mv#>
       PREFIX myext: <http://mycompany.com/metaverse-extension#>

       SELECT ?concept
       WHERE {
         ?concept a mv:PhysicalObject .
       }
   """

   results = g.query(query)
   # Results will include myext:HolographicDisplay
   ```

---

## Troubleshooting

### Common Issues

#### Issue 1: Ontology File Not Found

**Symptom**: Error loading `metaverse-ontology.ofn` or `.ttl` file

**Solution**:
1. Ensure you've run the extractor:
   ```bash
   ./logseq-owl-extractor/target/release/logseq-owl-extractor \
     --input . \
     --output metaverse-ontology.ofn
   ```

2. Convert to desired format:
   ```bash
   robot convert --input metaverse-ontology.ofn \
                 --output metaverse-ontology.ttl
   ```

3. Verify file exists:
   ```bash
   ls -lh metaverse-ontology.*
   ```

#### Issue 2: SPARQL Query Returns No Results

**Symptom**: Query executes but returns empty result set

**Solutions**:

1. **Check namespace prefixes**:
   ```sparql
   # Ensure you're using the correct namespace
   PREFIX mv: <http://www.metaverse-ontology.org/mv#>
   ```

2. **Verify ontology is loaded**:
   ```sparql
   # Count all triples
   SELECT (COUNT(*) AS ?count) WHERE { ?s ?p ?o }
   ```

3. **Use wildcard patterns to explore**:
   ```sparql
   # Find all predicates
   SELECT DISTINCT ?p WHERE { ?s ?p ?o } LIMIT 100
   ```

4. **Check for reasoning**:
   - Some queries require reasoning to be enabled
   - For Fuseki, ensure you're using a TDB dataset with inference

#### Issue 3: Protégé Won't Open Ontology

**Symptom**: Protégé shows parsing errors

**Solutions**:

1. **Validate syntax**:
   ```bash
   # Use whelk-rs to validate
   whelk validate metaverse-ontology.ofn
   ```

2. **Try different format**:
   ```bash
   # Convert to RDF/XML (more compatible)
   robot convert --input metaverse-ontology.ofn \
                 --output metaverse-ontology.owl
   ```

3. **Check for special characters**:
   - Ensure all IRIs are properly escaped
   - No spaces or special characters in concept names

#### Issue 4: Reasoner Takes Too Long

**Symptom**: Classification or reasoning is extremely slow

**Solutions**:

1. **Use a faster reasoner**:
   - whelk-rs (Rust-based, very fast)
   - ELK (optimized for EL profile)
   - Openllet (supports more features)

2. **Simplify the ontology**:
   ```sparql
   # Extract a module
   robot extract --method BOT \
                 --input metaverse-ontology.ofn \
                 --term mv:Avatar \
                 --output avatar-module.ofn
   ```

3. **Check for complex axioms**:
   - Avoid universal quantification when possible
   - Minimize cardinality restrictions

#### Issue 5: Integration Library Errors

**Symptom**: Python/JavaScript/Java code throws exceptions

**Solutions**:

1. **Python rdflib**:
   ```bash
   # Ensure latest version
   pip install --upgrade rdflib

   # Install additional parsers
   pip install rdflib-jsonld
   ```

2. **JavaScript rdf.js**:
   ```bash
   # Install all required packages
   npm install rdf-ext rdf-parser-n3 rdf-fetch rdflib
   ```

3. **Java Jena**:
   ```xml
   <!-- Use complete bundle -->
   <dependency>
       <groupId>org.apache.jena</groupId>
       <artifactId>apache-jena-libs</artifactId>
       <version>4.10.0</version>
       <type>pom</type>
   </dependency>
   ```

### Getting Help

#### Documentation

- **Project README**: `/home/john/githubs/OntologyDesign/README.md`
- **Quick Start Guide**: `/home/john/githubs/OntologyDesign/docs/guides/QUICKSTART.md`
- **Migration Guide**: `/home/john/githubs/OntologyDesign/docs/guides/MIGRATION_GUIDE.md`
- **Format Specification**: `/home/john/githubs/OntologyDesign/docs/reference/FORMAT_STANDARDIZED.md`

#### External Resources

- **OWL 2 Primer**: [https://www.w3.org/TR/owl2-primer/](https://www.w3.org/TR/owl2-primer/)
- **SPARQL Tutorial**: [https://www.w3.org/TR/sparql11-query/](https://www.w3.org/TR/sparql11-query/)
- **Protégé User Guide**: [https://protegewiki.stanford.edu/wiki/Main_Page](https://protegewiki.stanford.edu/wiki/Main_Page)
- **Apache Jena Documentation**: [https://jena.apache.org/documentation/](https://jena.apache.org/documentation/)
- **rdflib Documentation**: [https://rdflib.readthedocs.io/](https://rdflib.readthedocs.io/)

#### Community

- **ETSI Metaverse Standards**: [https://www.etsi.org/committee/metaverse](https://www.etsi.org/committee/metaverse)
- **Web3D Consortium**: [https://www.web3d.org/](https://www.web3d.org/)
- **Semantic Web Community**: [https://www.w3.org/2001/sw/](https://www.w3.org/2001/sw/)

#### Issue Reporting

For issues specific to this ontology:
1. Check existing documentation in `/home/john/githubs/OntologyDesign/docs/`
2. Review commit history for recent changes
3. File an issue in the project's issue tracker (if available)

---

## Appendix: File Locations

### Core Files

- **Ontology Definition**: `/home/john/githubs/OntologyDesign/OntologyDefinition.md`
- **Property Schema**: `/home/john/githubs/OntologyDesign/PropertySchema.md`
- **ETSI Domains**: `/home/john/githubs/OntologyDesign/ETSIDomainClassification.md`

### Examples

- **Avatar (VirtualAgent)**: `/home/john/githubs/OntologyDesign/Avatar.md`
- **Digital Twin (HybridObject)**: `/home/john/githubs/OntologyDesign/DigitalTwin.md`

### Tools

- **Extractor**: `/home/john/githubs/OntologyDesign/logseq-owl-extractor/`
- **Conversion Script**: `/home/john/githubs/OntologyDesign/convert_owl_to_ttl.py`

### Documentation

- **Guides**: `/home/john/githubs/OntologyDesign/docs/guides/`
- **Reference**: `/home/john/githubs/OntologyDesign/docs/reference/`

---

**Happy Ontologizing!**

For questions or contributions, please refer to the project README and documentation.
