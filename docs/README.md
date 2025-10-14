# Documentation

Complete documentation for the Metaverse Ontology project.

## 📖 Getting Started

Start here if you're new to the project:

| Document | Description |
|----------|-------------|
| [guides/QUICKSTART.md](guides/QUICKSTART.md) | Get up and running in 5 minutes |
| [guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md) | How to migrate VisioningLab concepts to the standard format |

## 📚 Reference Documentation

Technical reference materials:

| Document | Description |
|----------|-------------|
| [reference/TEMPLATE.md](reference/TEMPLATE.md) | Standard concept format template - copy this to create new concepts |
| [reference/FORMAT_STANDARDIZED.md](reference/FORMAT_STANDARDIZED.md) | Complete format specification and design rationale |
| [reference/URIMapping.md](reference/URIMapping.md) | Wikilink to IRI conversion rules |
| [reference/LOGSEQ_TAG_USAGE.md](reference/LOGSEQ_TAG_USAGE.md) | Using `metaverseOntology:: true` tag for Logseq queries |

## 📊 Project Status

Track progress and implementation:

| Document | Description |
|----------|-------------|
| [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) | All 6 issues from task.md resolved ✅ |
| [MIGRATION_STATUS.md](MIGRATION_STATUS.md) | Progress on migrating 260+ VisioningLab concepts |
| [task.md](task.md) | Original design requirements and analysis |

## 🎯 Key Concepts

### Orthogonal Classification

The ontology uses two independent dimensions to classify all metaverse concepts:

**Physicality Dimension:**
- **PhysicalEntity** - Has physical form (e.g., VR Headset, Sensor)
- **VirtualEntity** - Purely digital (e.g., Avatar, 3D Model, Software)
- **HybridEntity** - Binds physical and virtual (e.g., Digital Twin, AR Overlay)

**Role Dimension:**
- **Agent** - Autonomous decision-maker (e.g., Avatar, AI Assistant)
- **Object** - Passive, can be acted upon (e.g., 3D Model, Hardware, Data)
- **Process** - Activity or transformation (e.g., Rendering, Authentication)

**Inferred Classes (9 total):**

| Physicality | Role | → Inferred Class |
|-------------|------|------------------|
| Physical | Agent | PhysicalAgent |
| Physical | Object | PhysicalObject |
| Physical | Process | PhysicalProcess |
| Virtual | Agent | **VirtualAgent** |
| Virtual | Object | **VirtualObject** |
| Virtual | Process | **VirtualProcess** |
| Hybrid | Agent | HybridAgent |
| Hybrid | Object | **HybridObject** |
| Hybrid | Process | HybridProcess |

### ETSI Domains

Functional classification based on ETSI metaverse standards:

1. **InfrastructureDomain** - Network, compute, cloud, edge
2. **InteractionDomain** - UX, avatars, immersion, presence
3. **TrustAndGovernanceDomain** - Identity, security, privacy
4. **ComputationAndIntelligenceDomain** - AI, analytics, data processing
5. **CreativeMediaDomain** - 3D content, rendering, authoring
6. **VirtualEconomyDomain** - Tokens, NFTs, markets, transactions
7. **VirtualSocietyDomain** - Communities, governance, social

### Format Structure

Each concept file follows this structure:

```markdown
- ### OntologyBlock
  id:: [concept]-ontology
  collapsed:: true
	- metaverseOntology:: true        ← Tag for Logseq queries
	- term-id:: [unique number]
	- preferred-term:: [Official Name]
	- definition:: [Clear description]
	- owl:class:: mv:[ClassName]
	- owl:physicality:: [Physical/Virtual/Hybrid]Entity
	- owl:role:: [Agent/Object/Process]
	- owl:inferred-class:: mv:[InferredClass]
	- belongsToDomain:: [[ETSIDomain]]
	- #### Relationships
	  id:: [concept]-relationships
		- has-part:: [[Component]]
		- requires:: [[Dependency]]
		- enables:: [[Capability]]
	- #### OWL Axioms
	  id:: [concept]-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:[ClassName]))
		  SubClassOf(...)
		  ```
- ## About [Concept Name]
	- Human-readable description...
```

## 📂 File Organization

```
OntologyDesign/
├── docs/                              ← You are here
│   ├── README.md                      ← This file
│   ├── guides/                        ← How-to guides
│   ├── reference/                     ← Reference documentation
│   ├── IMPLEMENTATION_STATUS.md       ← Feature status
│   ├── MIGRATION_STATUS.md            ← Migration progress
│   └── task.md                        ← Original requirements
│
├── [Root directory - Core Files]
│   ├── OntologyDefinition.md         ← Ontology header & base classes
│   ├── PropertySchema.md             ← All properties
│   ├── ETSIDomainClassification.md   ← ETSI domains
│   ├── ValidationTests.md            ← Test cases
│   ├── Avatar.md                     ← Example: VirtualAgent
│   ├── DigitalTwin.md                ← Example: HybridObject
│   └── VisioningLab/                 ← 260+ concepts
│
└── logseq-owl-extractor/             ← Rust extraction tool
```

## 🔗 Quick Links

### Examples
- [../Avatar.md](../Avatar.md) - VirtualAgent example
- [../DigitalTwin.md](../DigitalTwin.md) - HybridObject example
- [../VisioningLab/Game Engine.md](../VisioningLab/Game%20Engine.md) - VirtualObject example

### Core Definitions
- [../OntologyDefinition.md](../OntologyDefinition.md) - Base classes and 9 intersection classes
- [../PropertySchema.md](../PropertySchema.md) - Object, data, and annotation properties
- [../ETSIDomainClassification.md](../ETSIDomainClassification.md) - ETSI domain taxonomy

### Tools
- [../logseq-owl-extractor/](../logseq-owl-extractor/) - Rust extraction tool

## 🎓 Learning Path

### 1. Understand the Basics (15 minutes)
1. Read the main [../README.md](../README.md)
2. Review the [orthogonal classification](#orthogonal-classification) concept
3. Study [../Avatar.md](../Avatar.md) as a complete example

### 2. Try It Out (30 minutes)
1. Follow [guides/QUICKSTART.md](guides/QUICKSTART.md)
2. Build the extractor tool
3. Extract OWL from the example files
4. View the result in Protégé (optional)

### 3. Create Your First Concept (1 hour)
1. Copy [reference/TEMPLATE.md](reference/TEMPLATE.md)
2. Follow the [guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md)
3. Classify your concept using the decision trees
4. Validate with the extractor tool

### 4. Deep Dive (Ongoing)
1. Read [reference/FORMAT_STANDARDIZED.md](reference/FORMAT_STANDARDIZED.md)
2. Learn about [reference/LOGSEQ_TAG_USAGE.md](reference/LOGSEQ_TAG_USAGE.md)
3. Study [reference/URIMapping.md](reference/URIMapping.md)
4. Review [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for all features

## 💬 Need Help?

- **Format questions?** See [reference/FORMAT_STANDARDIZED.md](reference/FORMAT_STANDARDIZED.md)
- **Classification questions?** See [guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md)
- **Tool questions?** See [../logseq-owl-extractor/README.md](../logseq-owl-extractor/README.md)
- **Implementation questions?** See [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md)

## 🎯 Next Steps

Based on where you are in the project:

**New Contributors:**
→ Start with [guides/QUICKSTART.md](guides/QUICKSTART.md)

**Migrating VisioningLab Files:**
→ Use [reference/TEMPLATE.md](reference/TEMPLATE.md) and [guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md)

**Checking Progress:**
→ Review [MIGRATION_STATUS.md](MIGRATION_STATUS.md)

**Understanding Design Decisions:**
→ Read [task.md](task.md) and [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md)

---

**Documentation Version:** 1.0
**Last Updated:** 2025-01-14
**Format Version:** Logseq-native outline with collapsed blocks
