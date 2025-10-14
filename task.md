# VisioningLab Migration Task - Agent Swarm Instructions

## 🎯 Mission

Migrate 260+ concept files in the `/VisioningLab` directory to the standardized Logseq + OWL format. This is a coordinated effort requiring systematic classification, format conversion, and validation.

## 📋 Current Status

- **Total Files**: 260+ markdown files in VisioningLab/
- **Completed**: 3 files (Avatar.md, DigitalTwin.md, Game Engine.md)
- **Remaining**: 257+ files
- **Target**: 100% conformant to specification

See [MIGRATION_STATUS.md](MIGRATION_STATUS.md) for detailed progress tracking.

---

## 🔧 Agent Swarm Coordination Protocol

### Agent Roles

**Classifier Agent** - Analyzes concepts and determines orthogonal classification
**Formatter Agent** - Converts files to standard format structure
**OWL Agent** - Creates formal OWL axioms based on classification
**Validator Agent** - Runs extractor tool to verify compliance
**Coordinator Agent** - Manages workflow and tracks progress

### Work Distribution

Each agent should claim a batch of files by updating MIGRATION_STATUS.md with their agent ID before starting work.

**Batch Size**: 5-10 files per agent at a time
**Status Updates**: Update MIGRATION_STATUS.md after each batch completion

---

## 📚 Required Reading Before Starting

All agents MUST review these reference documents:

1. **[reference/TEMPLATE.md](reference/TEMPLATE.md)** - The exact format structure to follow
2. **[reference/FORMAT_STANDARDIZED.md](reference/FORMAT_STANDARDIZED.md)** - Complete specification
3. **[guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md)** - Classification decision trees
4. **[reference/URIMapping.md](reference/URIMapping.md)** - Wikilink to IRI conversion rules

### Exemplar Files (Study These!)

- **[../Avatar.md](../Avatar.md)** - VirtualAgent example
- **[../DigitalTwin.md](../DigitalTwin.md)** - HybridObject example
- **[../VisioningLab/Game Engine.md](../VisioningLab/Game%20Engine.md)** - VirtualObject example

---

## 🔄 Migration Workflow

### Step 1: File Selection

```bash
# List all VisioningLab files
ls VisioningLab/*.md | sort
```

Choose a batch of 5-10 files and claim them in MIGRATION_STATUS.md:

```markdown
### Batch [N] - [Agent Name/ID]
Status: In Progress
Started: 2025-10-14

- [ ] File1.md
- [ ] File2.md
- [ ] File3.md
...
```

### Step 2: Read Original File

```bash
# Read the current file
cat "VisioningLab/[filename].md"
```

Analyze the content to understand:
- What is this concept?
- What are its key characteristics?
- How does it relate to other concepts?

### Step 3: Classify the Concept

Use the **Orthogonal Classification System**:

#### Dimension 1: Physicality

Ask: **"Does this concept have physical form?"**

- **PhysicalEntity** - Has atoms, occupies space (VR Headset, Sensor, Human)
- **VirtualEntity** - Purely digital, no physical form (Avatar, Software, 3D Model, Data)
- **HybridEntity** - Binds physical and virtual (Digital Twin, AR Overlay, Mixed Reality System)

#### Dimension 2: Role

Ask: **"What is this concept's primary function?"**

- **Agent** - Makes autonomous decisions, has agency (Avatar, AI Assistant, Bot, Human)
- **Object** - Passive, can be acted upon (Hardware, Software, Data, 3D Model, Tool)
- **Process** - Activity, transformation, operation (Rendering, Authentication, Synchronization)

#### Inferred Class Matrix

| Physicality | Role | → Inferred Class | Examples |
|-------------|------|------------------|----------|
| Physical | Agent | PhysicalAgent | Human, Robot |
| Physical | Object | PhysicalObject | VR Headset, Sensor, Server |
| Physical | Process | PhysicalProcess | Manufacturing, Physical Assembly |
| Virtual | Agent | **VirtualAgent** | Avatar, AI Bot, NPC |
| Virtual | Object | **VirtualObject** | 3D Model, Software, Game Engine |
| Virtual | Process | **VirtualProcess** | Rendering, Authentication, Pathfinding |
| Hybrid | Agent | HybridAgent | Cyborg, Teleoperated Robot |
| Hybrid | Object | **HybridObject** | Digital Twin, AR Overlay |
| Hybrid | Process | HybridProcess | AR Rendering, IoT Data Sync |

**Common Patterns:**

- **Software/Tools** → VirtualObject
- **3D Assets/Models** → VirtualObject
- **Algorithms/Computations** → VirtualProcess
- **User Representations** → VirtualAgent
- **Hardware Devices** → PhysicalObject
- **Anything with "Digital Twin" semantics** → HybridObject

### Step 4: Assign ETSI Domain

Choose at least one domain from [../ETSIDomainClassification.md](../ETSIDomainClassification.md):

1. **InfrastructureDomain** - Networks, compute, cloud, edge
2. **InteractionDomain** - UX, avatars, immersion, interfaces
3. **TrustAndGovernanceDomain** - Identity, security, privacy
4. **ComputationAndIntelligenceDomain** - AI, ML, analytics
5. **CreativeMediaDomain** - 3D content, rendering, audio/video
6. **VirtualEconomyDomain** - NFTs, tokens, transactions
7. **VirtualSocietyDomain** - Communities, social structures

**Can assign multiple domains** if concept spans multiple areas.

### Step 5: Assign Unique term-id

Check [MIGRATION_STATUS.md](MIGRATION_STATUS.md) for the next available term-id number.

- Start from 20100 and increment for each new file
- Update MIGRATION_STATUS.md immediately after claiming a term-id
- **Do not reuse term-ids**

### Step 6: Create Conformant File

Use this exact structure (copy from [reference/TEMPLATE.md](reference/TEMPLATE.md)):

```markdown
- ### OntologyBlock
  id:: [concept-kebab-case]-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: [unique number]
	- preferred-term:: [Official Name]
	- definition:: [Clear 1-2 sentence definition]
	- maturity:: [draft|mature|deprecated]
	- source:: [[Original Source or Standard]]
	- owl:class:: mv:[ClassName]
	- owl:physicality:: [Physical|Virtual|Hybrid]Entity
	- owl:role:: [Agent|Object|Process]
	- owl:inferred-class:: mv:[PhysicalityRole]
	- owl:functional-syntax:: true
	- belongsToDomain:: [[DomainName]]
	- implementedInLayer:: [[OptionalArchitectureLayer]]
	- #### Relationships
	  id:: [concept-kebab-case]-relationships
		- has-part:: [[Component1]], [[Component2]]
		- requires:: [[Dependency1]], [[Dependency2]]
		- enables:: [[Capability1]], [[Capability2]]
		- related-to:: [[RelatedConcept1]]
	- #### OWL Axioms
	  id:: [concept-kebab-case]-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:[ClassName]))

		  # Classification axioms
		  SubClassOf(mv:[ClassName] mv:[Physicality]Entity)
		  SubClassOf(mv:[ClassName] mv:[Role])

		  # Additional axioms as needed
		  # (cardinality constraints, property restrictions, etc.)
		  ```
- ## About [Concept Name]
  id:: [concept-kebab-case]-about
	- [Human-readable description, examples, use cases]
	-
	- ### Key Characteristics
		- [Important feature 1]
		- [Important feature 2]
	-
	- ### Common Implementations
		- [Example 1]
		- [Example 2]
	-
	- ### Standards and References
		- [Relevant standard or paper if applicable]
```

### Step 7: Naming Conventions

**IRI Convention (kebab-case → camelCase):**
- File: `Game Engine.md` → Class: `mv:GameEngine`
- File: `3D Model.md` → Class: `mv:ThreeDModel`
- File: `AR Glasses.md` → Class: `mv:ARGlasses`

**Section IDs (kebab-case):**
- `Game Engine` → `game-engine-ontology`
- `3D Model` → `3d-model-ontology`
- `AR Glasses` → `ar-glasses-ontology`

**Relationships (kebab-case properties):**
- Use: `has-part::`, `requires::`, `enables::`, `related-to::`

### Step 8: OWL Axioms - Minimum Required

Every concept MUST have at minimum:

```clojure
Declaration(Class(mv:[ClassName]))

# Classification - REQUIRED
SubClassOf(mv:[ClassName] mv:[Physicality]Entity)
SubClassOf(mv:[ClassName] mv:[Role])
```

**For concepts with strong constraints, add:**

```clojure
# Example: Avatar must represent exactly one agent
SubClassOf(mv:Avatar
  ObjectExactCardinality(1 mv:represents mv:Agent)
)

# Example: Digital Twin must synchronize with physical entity
SubClassOf(mv:DigitalTwin
  ObjectSomeValuesFrom(mv:synchronizesWith mv:PhysicalEntity)
)
```

### Step 9: Validate

Run the extractor tool to validate your changes:

```bash
cd logseq-owl-extractor
cargo build --release

# Test extraction on your batch
./target/release/logseq-owl-extractor \
  --input ../VisioningLab \
  --output test-batch.ofn \
  --validate
```

**Common Errors to Check:**
- ✅ Syntax errors in OWL axioms
- ✅ Missing required properties (metaverseOntology, term-id, owl:class, owl:physicality, owl:role)
- ✅ Incorrect indentation (use tabs for Logseq outline)
- ✅ Missing section IDs
- ✅ Mismatched inferred class (VirtualAgent but classified as Physical)

### Step 10: Update Progress

Mark files complete in [MIGRATION_STATUS.md](MIGRATION_STATUS.md):

```markdown
### Batch [N] - [Agent Name/ID]
Status: Complete ✅
Completed: 2025-10-14

- [x] File1.md - VirtualObject
- [x] File2.md - VirtualProcess
- [x] File3.md - PhysicalObject
```

---

## 🚦 Quality Checklist

Before marking a file complete, verify:

- [ ] File uses `- ### OntologyBlock` heading (level 3)
- [ ] `collapsed:: true` property set
- [ ] `metaverseOntology:: true` is FIRST property
- [ ] Unique `term-id::` assigned and recorded
- [ ] Clear `definition::` provided
- [ ] `owl:class::` uses proper camelCase naming
- [ ] `owl:physicality::` is one of: PhysicalEntity, VirtualEntity, HybridEntity
- [ ] `owl:role::` is one of: Agent, Object, Process
- [ ] `owl:inferred-class::` correctly combines physicality + role
- [ ] At least one `belongsToDomain::` assigned
- [ ] Section IDs follow pattern: `[concept]-ontology`, `[concept]-relationships`, `[concept]-owl-axioms`
- [ ] OWL axioms use ```clojure syntax highlighting
- [ ] OWL axioms include minimum required: Declaration + 2x SubClassOf
- [ ] All wikilinks use `[[Double Brackets]]` format
- [ ] File extracts without errors using logseq-owl-extractor
- [ ] Human-readable "About" section included

---

## 🎯 Prioritization Strategy

### Phase 1: Simple VirtualObjects (Weeks 1-2)
**Target: 40 files**

Focus on straightforward software and tools:
- Game engines, rendering engines
- 3D modeling software
- Development tools
- File formats
- Data structures

**Rationale:** These are easiest to classify (almost all VirtualObject) and build confidence.

### Phase 2: Infrastructure & Hardware (Weeks 3-4)
**Target: 50 files**

Physical devices and infrastructure:
- VR/AR headsets
- Sensors and IoT devices
- Networking equipment
- Servers and compute

**Rationale:** Clear PhysicalObject classification, well-defined relationships.

### Phase 3: Virtual Processes (Weeks 5-6)
**Target: 40 files**

Algorithms and computational processes:
- Rendering techniques
- Authentication methods
- Data processing
- Optimization algorithms

**Rationale:** Requires understanding of what makes something a "Process" vs "Object".

### Phase 4: Agents & Hybrids (Weeks 7-8)
**Target: 30 files**

More complex classifications:
- AI agents, bots, NPCs
- Digital twins
- AR/MR hybrid systems
- Teleoperation systems

**Rationale:** Requires careful analysis of agency and hybrid semantics.

### Phase 5: Domain-Specific Concepts (Weeks 9-10)
**Target: 60 files**

Specialized concepts:
- Virtual economy concepts
- Social systems
- Creative tools
- Governance mechanisms

**Rationale:** May require domain expertise and multiple ETSI domains.

### Phase 6: Edge Cases & Cleanup (Week 11)
**Target: 37 remaining files**

Final pass:
- Ambiguous classifications
- Multi-domain concepts
- Deprecated or merged concepts
- Cross-reference validation

---

## 🤝 Agent Collaboration Patterns

### Pattern 1: Pipeline Processing

```
Classifier Agent → Identifies 10 files, classifies them
                ↓
Formatter Agent → Applies template structure
                ↓
OWL Agent → Adds formal axioms
                ↓
Validator Agent → Runs extractor, reports issues
                ↓
Coordinator → Updates MIGRATION_STATUS.md
```

### Pattern 2: Parallel Batches

```
Agent A: VisioningLab/[Files 1-10]
Agent B: VisioningLab/[Files 11-20]
Agent C: VisioningLab/[Files 21-30]
Agent D: VisioningLab/[Files 31-40]

Each agent completes full workflow independently
```

### Pattern 3: Specialized Teams

```
Team 1: PhysicalObjects only
Team 2: VirtualObjects only
Team 3: Processes only
Team 4: Complex cases (Agents, Hybrids)
```

### Pattern 4: Review & Validate

```
Agent X: Migrates batch of 10 files
Agent Y: Reviews and validates Agent X's work
Agent Z: Runs full extraction test on batch
```

---

## 📊 Progress Tracking

### Daily Status Update Format

Update [MIGRATION_STATUS.md](MIGRATION_STATUS.md) daily:

```markdown
## Week [N] - [Date Range]

### Daily Progress

**Day 1 (2025-10-14)**
- Agent A: Completed Batch 1 (10 files) - VirtualObjects
- Agent B: In Progress Batch 2 (8/10 files) - PhysicalObjects
- Agent C: Completed Batch 3 (5 files) - VirtualProcesses
- **Total Today**: 15 files completed
- **Running Total**: 18 / 260 (6.9%)

**Day 2 (2025-10-15)**
...
```

### Weekly Milestone Tracking

```markdown
## Milestones

- [ ] 10% complete (26 files) - Target: Week 1
- [ ] 25% complete (65 files) - Target: Week 3
- [ ] 50% complete (130 files) - Target: Week 6
- [ ] 75% complete (195 files) - Target: Week 9
- [ ] 100% complete (260 files) - Target: Week 11
```

---

## ⚠️ Common Issues & Solutions

### Issue 1: Classification Ambiguity

**Problem**: "Is this concept an Object or a Process?"

**Decision Rule**:
- If it's primarily a **thing/tool/artifact** → Object
- If it's primarily an **activity/operation/transformation** → Process

**Examples**:
- "Rendering Engine" → VirtualObject (it's software)
- "Rendering" → VirtualProcess (it's the activity of producing images)
- "Authentication System" → VirtualObject (it's a component)
- "Authentication" → VirtualProcess (it's the activity of verifying identity)

### Issue 2: Missing Information

**Problem**: Original file lacks clear definition or context

**Solution**:
1. Search for concept in industry standards (ETSI, ISO, W3C)
2. Check Wikipedia or technical glossaries
3. Use definition:: "Draft definition requiring expert review"
4. Set maturity:: draft
5. Add comment in About section noting need for review

### Issue 3: Duplicate or Similar Concepts

**Problem**: Multiple files seem to describe the same or very similar concepts

**Solution**:
1. **Do not merge yet** - migrate each file separately
2. Add cross-references in "About" section: "See also [[Related Concept]]"
3. Document in MIGRATION_STATUS.md for later review
4. Flag for coordinator to decide on merging strategy

### Issue 4: Extractor Validation Fails

**Problem**: File fails to extract or validate

**Checklist**:
1. Check OWL syntax with online validator
2. Verify all required properties present
3. Check indentation (must use tabs, not spaces in outline)
3. Verify section IDs are unique within file
4. Check that class name matches across owl:class and OWL axioms
5. Ensure code fence uses ```clojure not ```owl

### Issue 5: term-id Collision

**Problem**: Two agents assign the same term-id

**Solution**:
1. Second agent must immediately change their term-id
2. Update MIGRATION_STATUS.md with correct mapping
3. Coordinator maintains authoritative term-id registry

---

## 🔍 Validation & Quality Assurance

### Level 1: Self-Validation (Each Agent)

```bash
# After completing each file
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input VisioningLab/[filename].md \
  --output test-[filename].ofn \
  --validate
```

### Level 2: Batch Validation (After 10 files)

```bash
# Validate entire batch together
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input VisioningLab \
  --output batch-test.ofn \
  --validate
```

### Level 3: Full Ontology Build (Weekly)

```bash
# Build complete ontology with all migrated files
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology-full.ofn \
  --validate

# Test with reasoner (if available)
whelk classify metaverse-ontology-full.ofn
```

### Level 4: Peer Review (Sample 10%)

Random sample of 10% of completed files reviewed by different agent:
- Check classification correctness
- Verify format compliance
- Validate OWL axiom quality
- Review definition clarity

---

## 📈 Success Metrics

### Quantitative Metrics

- **Files completed** / 260 total
- **Validation pass rate** (should be 100%)
- **Average time per file** (target: 15-20 minutes)
- **Daily throughput** (target: 20-30 files/day with 4 agents)

### Qualitative Metrics

- **Classification consistency** - Similar concepts classified similarly
- **Definition clarity** - Definitions are clear and precise
- **Axiom quality** - OWL axioms capture essential constraints
- **Documentation completeness** - About sections are informative

---

## 🎓 Learning Resources

### Before You Start

1. Read [../README.md](../README.md) - Understand the project
2. Study [reference/FORMAT_STANDARDIZED.md](reference/FORMAT_STANDARDIZED.md) - Know the spec
3. Review all 3 exemplar files - See what "good" looks like

### During Migration

1. Keep [reference/TEMPLATE.md](reference/TEMPLATE.md) open for copy-paste
2. Reference [guides/MIGRATION_GUIDE.md](guides/MIGRATION_GUIDE.md) for classification
3. Check [reference/URIMapping.md](reference/URIMapping.md) for naming rules

### For OWL Axioms

1. [OWL 2 Primer](https://www.w3.org/TR/owl2-primer/) - Official W3C documentation
2. [OWL Functional Syntax](https://www.w3.org/TR/owl2-syntax/) - Syntax reference
3. Study axioms in Avatar.md and DigitalTwin.md - Real examples

---

## 🚀 Getting Started - First Steps

### For New Agent Joining the Swarm

1. **Read This Entire Document** (20 minutes)
2. **Study the 3 Exemplars** (30 minutes)
   - Avatar.md
   - DigitalTwin.md
   - Game Engine.md
3. **Build the Extractor Tool** (10 minutes)
   ```bash
   cd logseq-owl-extractor
   cargo build --release
   ```
4. **Claim Your First Batch** (5 minutes)
   - Choose 5 simple VirtualObject files
   - Update MIGRATION_STATUS.md with your agent ID and file list
5. **Start Migrating!** (2-3 hours for first batch)

### Your First Batch - Recommended Files

Start with these simple VirtualObject concepts:
- VisioningLab/[pick 5 software/tool concepts]

These will help you learn the workflow without complex classification decisions.

---

## 📞 Coordination & Communication

### Status Updates Required

- **Start of shift**: Claim batch in MIGRATION_STATUS.md
- **Every 10 files**: Update progress
- **End of shift**: Mark completed, note any issues
- **Blockers**: Immediately flag in MIGRATION_STATUS.md

### Decision Escalation

**Level 1 - Self-resolve**: Minor formatting, obvious classifications
**Level 2 - Peer consult**: Classification ambiguity, missing info
**Level 3 - Coordinator**: Duplicate concepts, structural changes, spec clarifications

### File Naming for Outputs

```
MIGRATION_STATUS.md - Progress tracking (shared)
agent-[id]-notes.md - Personal notes (optional)
batch-[n]-validation.txt - Validation output (keep for reference)
```

---

## ✅ Definition of Done

A file migration is complete when:

1. ✅ File follows exact template structure
2. ✅ All required properties present and correctly formatted
3. ✅ Classification validated against decision trees
4. ✅ OWL axioms include minimum required declarations
5. ✅ File extracts successfully with logseq-owl-extractor
6. ✅ term-id recorded in MIGRATION_STATUS.md
7. ✅ File marked complete in MIGRATION_STATUS.md
8. ✅ Git commit created with clear message
9. ✅ No extractor errors or warnings
10. ✅ About section provides human context

---

## 🎯 Final Goal

**260+ standardized, validated, extractable concept files that:**
- Form a complete, consistent metaverse ontology
- Can be queried in Logseq using metaverseOntology:: true tag
- Extract to valid OWL 2 DL for formal reasoning
- Serve as reference for metaverse standards development
- Enable automatic classification through reasoner inference

---

**Task Document Version**: 2.0
**Last Updated**: 2025-10-14
**Coordinator**: [To be assigned]
**Estimated Completion**: 11 weeks with 4-agent swarm

**Let's build the metaverse ontology! 🚀**
