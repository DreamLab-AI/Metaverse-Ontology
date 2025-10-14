- ### OntologyBlock
  id:: [concept]-ontology
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
	- owl:functional-syntax:: true
	- belongsToDomain:: [[ETSIDomainName]]
	- implementedInLayer:: [[ArchitectureLayerName]]
	- #### Relationships
	  id:: [concept]-relationships
		- has-part:: [[Component 1]], [[Component 2]]
		- is-part-of:: [[Parent Concept]]
		- requires:: [[Dependency 1]], [[Dependency 2]]
		- depends-on:: [[Related Dependency]]
		- enables:: [[Enabled Capability 1]], [[Enabled Capability 2]]
		- binds-to:: [[Physical Entity]], [[Virtual Entity]]
		  *Note: only for HybridEntity concepts*
	- #### OWL Axioms
	  id:: [concept]-owl-axioms
	  collapsed:: true
		- ```clojure
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
  id:: [concept]-about
	- [Opening paragraph with high-level description and context]
	- ### Key Characteristics
	  id:: [concept]-characteristics
		- [Characteristic 1]
		- [Characteristic 2]
		- [Characteristic 3]
		- [Characteristic 4]
	- ### Technical Components
	  id:: [concept]-components
		- [[Component 1]] - Description of component
		- [[Component 2]] - Description of component
		- [[Component 3]] - Description of component
		- Additional technical details
	- ### Functional Capabilities
	  id:: [concept]-capabilities
		- **[Capability 1]**: Description of what it enables
		- **[Capability 2]**: Description of what it enables
		- **[Capability 3]**: Description of what it enables
		- **[Capability 4]**: Description of what it enables
	- ### Use Cases
	  id:: [concept]-use-cases
		- Use case 1 with specific examples
		- Use case 2 with specific examples
		- Use case 3 with specific examples
		- Industry applications
		- Real-world implementations
	- ### Standards & References
	  id:: [concept]-standards
		- [[Standard 1]] - Description
		- [[Standard 2]] - Description
		- Industry specifications
		- Research papers or documentation
	- ### Related Concepts
	  id:: [concept]-related
		- [[Related Concept 1]] - How it relates
		- [[Related Concept 2]] - How it relates
		- [[Related Concept 3]] - How it relates
		- [[Inferred Parent Class]] - Ontology classification

---

## Template Usage Guide

### Visual Structure in Logseq

When collapsed (default view):
```
ConceptName (from filename)
├─ ### OntologyBlock (collapsed) ← All technical details hidden
└─ ## About [Concept] (expanded) ← Human-readable always visible
```

When OntologyBlock expanded:
```
ConceptName
├─ ### OntologyBlock
│  ├─ Properties (term-id, owl:class, etc.)
│  ├─ #### Relationships (collapsible)
│  └─ #### OWL Axioms (collapsed by default)
└─ ## About [Concept]
   ├─ Key Characteristics
   └─ ...
```

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
- `owl:functional-syntax:: true` - Flag indicating OWL axioms are present

**Include when applicable:**
- `source` - Standards body or source
- `belongsToDomain` - ETSI domain(s)
- `implementedInLayer` - Architecture layer(s)

### Section IDs

Use consistent ID naming:
- `id:: [concept]-ontology` - Main OntologyBlock
- `id:: [concept]-relationships` - Relationships section
- `id:: [concept]-owl-axioms` - OWL Axioms section
- `id:: [concept]-about` - About section
- `id:: [concept]-characteristics` - Key Characteristics
- `id:: [concept]-components` - Technical Components
- `id:: [concept]-capabilities` - Functional Capabilities
- `id:: [concept]-use-cases` - Use Cases
- `id:: [concept]-standards` - Standards & References
- `id:: [concept]-related` - Related Concepts

Replace `[concept]` with lowercase concept name (e.g., `avatar`, `digitaltwin`, `gameengine`)

### Relationship Properties

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

### OWL Code Blocks

- Use triple backticks with `clojure` for syntax highlighting
- Include `owl:functional-syntax:: true` property as flag
- Actual OWL code goes in code block (no property syntax inside)
- Keep OWL Axioms subsection `collapsed:: true` by default

### Human-Readable Sections

The "About [Concept]" section is for:
- Contextual explanations for humans
- Real-world examples and use cases
- Technical details and implementation notes
- Cross-references to related concepts
- Standards and reference materials

**This section is NOT extracted by the parser** - it's purely for human consumption in Logseq.

### Examples

See these exemplar files:
- [Avatar.md](Avatar.md) - VirtualAgent example
- [DigitalTwin.md](DigitalTwin.md) - HybridObject example
- [VisioningLab/Game Engine.md](VisioningLab/Game%20Engine.md) - VirtualObject example

---

## Quick Start

1. Copy this template
2. Replace `[concept]` with lowercase concept name in all IDs
3. Replace filename with concept name (spaces OK)
4. Fill in all required fields in OntologyBlock
5. Classify using Physicality + Role dimensions
6. Add relationships as wikilinks
7. Write OWL axioms in code block with clojure syntax
8. Write human-readable "About" section
9. Add section IDs for all subsections
10. Test extraction with logseq-owl-extractor
11. Verify in Logseq for readability and collapsibility

---

## Key Advantages

✅ **Tidy**: Everything collapses into ### OntologyBlock
✅ **Queryable**: metaverseOntology tag enables Logseq queries
✅ **Referenceable**: Section IDs allow block references
✅ **Readable**: Clojure syntax highlighting for OWL code
✅ **Extractable**: Parser extracts properties and OWL blocks
✅ **Documented**: Human context in "About" section
✅ **Linked**: WikiLinks create knowledge graph

---

## Validation Checklist

Before committing a new concept file:

- [ ] Filename matches concept name (spaces OK)
- [ ] ### OntologyBlock heading (level 3)
- [ ] All section IDs follow `[concept]-section` pattern
- [ ] `collapsed:: true` on OntologyBlock
- [ ] `metaverseOntology:: true` is first property
- [ ] term-id is unique numeric value
- [ ] definition is clear and concise
- [ ] maturity is set (draft/mature/deprecated)
- [ ] `owl:functional-syntax:: true` flag present
- [ ] owl:physicality dimension is correct
- [ ] owl:role dimension is correct
- [ ] owl:inferred-class matches physicality+role combination
- [ ] At least one domain assigned
- [ ] Wikilinks use `[[Page Name]]` format
- [ ] OWL Axioms in code block with ```clojure
- [ ] OWL Axioms subsection is `collapsed:: true`
- [ ] OWL Functional Syntax is valid
- [ ] "About" section has useful human-readable content
- [ ] File extracts successfully with logseq-owl-extractor
- [ ] File looks clean and tidy when viewed in Logseq
