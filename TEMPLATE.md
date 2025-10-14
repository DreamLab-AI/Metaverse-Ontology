- OntologyBlock
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: [unique numeric ID]
	- preferred-term:: [Official Term Name]
	- definition:: [Clear, concise definition of the concept in one or two sentences]
	- maturity:: [draft|mature|deprecated]
	- source:: [[Primary Source or Standard]]
	- owl:class:: mv:[ClassName]
	- owl:physicality:: [PhysicalEntity|VirtualEntity|HybridEntity]
	- owl:role:: [Agent|Object|Process]
	- owl:inferred-class:: mv:[InferredClassName]
	- belongsToDomain:: [[ETSIDomainName]]
	- implementedInLayer:: [[ArchitectureLayerName]]
	- ## Relationships
		- has-part:: [[Component 1]], [[Component 2]]
		- is-part-of:: [[Parent Concept]]
		- requires:: [[Dependency 1]], [[Dependency 2]]
		- depends-on:: [[Related Dependency]]
		- enables:: [[Enabled Capability 1]], [[Enabled Capability 2]]
		- binds-to:: [[Physical Entity]], [[Virtual Entity]]
		  *Note: only for HybridEntity concepts*
	- ## OWL Axioms
	  collapsed:: true
		- ```
		  owl:functional-syntax:: |
		    Declaration(Class(mv:[ClassName]))

		    # Classification along two primary dimensions
		    SubClassOf(mv:[ClassName] mv:[PhysicalityDimension])
		    SubClassOf(mv:[ClassName] mv:[RoleDimension])

		    # Domain-specific constraints (add as needed)
		    # Example cardinality constraint:
		    # SubClassOf(mv:[ClassName]
		    #   ObjectExactCardinality(1 mv:propertyName mv:RangeClass)
		    # )

		    # Domain classification
		    SubClassOf(mv:[ClassName]
		      ObjectSomeValuesFrom(mv:belongsToDomain mv:[DomainName])
		    )

		    # Layer classification (if applicable)
		    SubClassOf(mv:[ClassName]
		      ObjectSomeValuesFrom(mv:implementedInLayer mv:[LayerName])
		    )
		  ```
- ## About [Concept Name]
	- [Opening paragraph with high-level description and context]
	- ### Key Characteristics
		- [Characteristic 1]
		- [Characteristic 2]
		- [Characteristic 3]
		- [Characteristic 4]
	- ### Technical Components
		- [[Component 1]] - Description of component
		- [[Component 2]] - Description of component
		- [[Component 3]] - Description of component
		- Additional technical details
	- ### Functional Capabilities
		- **[Capability 1]**: Description of what it enables
		- **[Capability 2]**: Description of what it enables
		- **[Capability 3]**: Description of what it enables
		- **[Capability 4]**: Description of what it enables
	- ### Use Cases
		- Use case 1 with specific examples
		- Use case 2 with specific examples
		- Use case 3 with specific examples
		- Industry applications
		- Real-world implementations
	- ### Standards & References
		- [[Standard 1]] - Description
		- [[Standard 2]] - Description
		- Industry specifications
		- Research papers or documentation
	- ### Related Concepts
		- [[Related Concept 1]] - How it relates
		- [[Related Concept 2]] - How it relates
		- [[Related Concept 3]] - How it relates
		- [[Inferred Parent Class]] - Ontology classification

---

## Template Usage Guide

### Required Fields

**Always include these:**
- `metaverseOntology:: true` - Logseq tag marking this as part of the ontology
- `term-id` - Unique numeric identifier
- `preferred-term` - Official term name
- `definition` - Clear one-sentence description
- `maturity` - Current status (draft/mature/deprecated)
- `owl:class` - OWL class IRI (mv:ClassName)
- `owl:physicality` - Physical/Virtual/Hybrid dimension
- `owl:role` - Agent/Object/Process dimension
- `owl:inferred-class` - What reasoner will infer

**Include when applicable:**
- `source` - Standards body or source
- `belongsToDomain` - ETSI domain(s)
- `implementedInLayer` - Architecture layer(s)

### Relationships

Use these properties as appropriate:
- `has-part` - Components or parts of this concept
- `is-part-of` - What this is a component of
- `requires` - Hard dependencies needed to function
- `depends-on` - Soft dependencies or related concepts
- `enables` - Capabilities or features this provides
- `binds-to` - Physical/Virtual bindings (HybridEntity only)

### OWL Classification Guide

#### Physicality Dimension

**PhysicalEntity** - Has physical form
- Examples: VR Headset, Motion Capture Rig, Edge Server, Sensor

**VirtualEntity** - Purely digital
- Examples: Software, 3D Model, Data, Protocol, Avatar, Smart Contract

**HybridEntity** - Binds physical and virtual
- Examples: Digital Twin, AR Overlay, IoT Device with Virtual Representation

#### Role Dimension

**Agent** - Autonomous decision-maker
- Examples: Avatar, AI Assistant, Autonomous Agent, Virtual Entity with agency

**Object** - Passive, can be acted upon
- Examples: 3D Model, Hardware, Data, Building, Asset, Document

**Process** - Activity or transformation
- Examples: Rendering, Authentication, Synchronization, Protocol Execution

#### Inferred Classes (9 combinations)

| Physicality | Role | Inferred Class |
|-------------|------|----------------|
| Physical | Agent | PhysicalAgent |
| Physical | Object | PhysicalObject |
| Physical | Process | PhysicalProcess |
| Virtual | Agent | VirtualAgent |
| Virtual | Object | VirtualObject |
| Virtual | Process | VirtualProcess |
| Hybrid | Agent | HybridAgent |
| Hybrid | Object | HybridObject |
| Hybrid | Process | HybridProcess |

### ETSI Domains

Choose one or more:
- `InfrastructureDomain` - Network, compute, cloud, edge
- `InteractionDomain` - UX, avatars, immersion, presence
- `TrustAndGovernanceDomain` - Identity, security, privacy
- `ComputationAndIntelligenceDomain` - AI, analytics, data processing
- `CreativeMediaDomain` - 3D content, rendering, authoring
- `VirtualEconomyDomain` - Tokens, NFTs, markets, transactions
- `VirtualSocietyDomain` - Communities, governance, social

### Human-Readable Sections

The "About [Concept]" section is for:
- Contextual explanations for humans
- Real-world examples and use cases
- Technical details and implementation notes
- Cross-references to related concepts
- Standards and reference materials

**This section is NOT extracted by the parser** - it's purely for human consumption in Logseq.

### Visual Appearance in Logseq

When collapsed, users see:
```
Concept Name (from filename)
├─ OntologyBlock (collapsed)
└─ About [Concept] (expanded)
```

When OntologyBlock is expanded:
```
Concept Name
├─ OntologyBlock
│  ├─ term-id: 12345
│  ├─ owl:physicality: VirtualEntity
│  ├─ owl:role: Object
│  ├─ Relationships (collapsed)
│  └─ OWL Axioms (collapsed)
└─ About [Concept]
   ├─ Key Characteristics
   ├─ Technical Components
   └─ ...
```

### Examples

See these exemplar files:
- [Avatar.md](Avatar.md) - VirtualAgent example
- [DigitalTwin.md](DigitalTwin.md) - HybridObject example
- [VisioningLab/Game Engine.md](VisioningLab/Game%20Engine.md) - VirtualObject example

---

## Quick Start

1. Copy this template
2. Replace filename with concept name
3. Fill in all required fields in OntologyBlock
4. Classify using Physicality + Role dimensions
5. Add relationships as wikilinks
6. Write OWL axioms in code fence
7. Write human-readable "About" section
8. Test extraction with logseq-owl-extractor
9. Verify in Logseq for readability

---

## Validation Checklist

Before committing a new concept file:

- [ ] Filename matches concept name (spaces OK)
- [ ] term-id is unique numeric value
- [ ] definition is clear and concise
- [ ] maturity is set (draft/mature/deprecated)
- [ ] owl:physicality dimension is correct
- [ ] owl:role dimension is correct
- [ ] owl:inferred-class matches physicality+role combination
- [ ] At least one domain assigned
- [ ] Wikilinks use `[[Page Name]]` format
- [ ] OWL Axioms block uses code fence with triple backticks
- [ ] OWL Functional Syntax is valid
- [ ] "About" section has useful human-readable content
- [ ] File extracts successfully with logseq-owl-extractor
- [ ] File looks clean when viewed in Logseq
