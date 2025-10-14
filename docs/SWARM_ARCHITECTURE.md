# Swarm Architecture Design - VisioningLab Ontology Migration

**Version**: 1.0
**Date**: 2025-10-14
**Status**: Ready for Implementation
**Target**: 271 remaining files in VisioningLab directory
**Estimated Duration**: 6 weeks
**Daily Throughput**: 60 files/day

---

## Executive Summary

This architecture defines a 5-agent swarm optimized for migrating 271 markdown files to the standardized Logseq + OWL format. The design leverages parallel processing, MCP memory coordination, and claude-flow hooks to achieve high throughput (60 files/day) while maintaining strict quality gates.

**Key Innovations**:
- **Parallel Pipeline**: 2 concurrent batch streams processing 30 files per 5-hour cycle
- **Memory-Based Coordination**: Agents communicate via MCP memory keys, eliminating direct coupling
- **Automated Quality Gates**: 4-level validation cascade ensures 100% extraction success
- **Phased Prioritization**: Simple→Complex progression builds agent confidence and pattern recognition

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        COORDINATOR AGENT                            │
│  - Term-ID Registry Management                                      │
│  - Batch Assignment & Progress Tracking                             │
│  - Quality Gate Enforcement                                         │
│  - Escalation Resolution                                            │
└────────────┬────────────────────────────────────────────┬───────────┘
             │                                            │
    ┌────────▼────────┐                          ┌───────▼──────┐
    │ Memory Bus      │                          │ MIGRATION_   │
    │ swarm/*         │◄─────────────────────────┤ STATUS.md    │
    └────────┬────────┘                          └──────────────┘
             │
    ┌────────▼────────────────────────────────────────────┐
    │            PARALLEL PROCESSING PIPELINE             │
    │                                                      │
    │  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │
    │  │ CLASSIFIER  │  │ CLASSIFIER  │  │  VALIDATOR │  │
    │  │  Agent A    │  │  Agent B    │  │   Agent    │  │
    │  │  (Batch 1)  │  │  (Batch 2)  │  │  (Both)    │  │
    │  └──────┬──────┘  └──────┬──────┘  └─────▲──────┘  │
    │         │                │                │         │
    │  ┌──────▼──────┐  ┌──────▼──────┐       │         │
    │  │ FORMATTER   │  │ FORMATTER   │       │         │
    │  │  Agent A    │  │  Agent B    │       │         │
    │  │  (Batch 1)  │  │  (Batch 2)  │       │         │
    │  └──────┬──────┘  └──────┬──────┘       │         │
    │         │                │                │         │
    │  ┌──────▼──────┐  ┌──────▼──────┐       │         │
    │  │ OWL AGENT   │  │ OWL AGENT   │       │         │
    │  │  Agent A    │  │  Agent B    │       │         │
    │  │  (Batch 1)  │  │  (Batch 2)  │       │         │
    │  └──────┬──────┘  └──────┬──────┘       │         │
    │         └────────┬────────┘              │         │
    │                  └───────────────────────┘         │
    └─────────────────────────────────────────────────────┘
```

---

## Agent Role Definitions

### 1. Classifier Agent (2 instances)

**Type**: Analyst
**Primary Responsibility**: Analyze concepts and determine orthogonal classification
**Batch Size**: 15 files
**Cycle Time**: 2 hours

**Key Tasks**:
1. Read and analyze concept files from VisioningLab/
2. Apply classification decision trees from guides/MIGRATION_GUIDE.md
3. Determine physicality dimension (Physical/Virtual/Hybrid)
4. Determine role dimension (Agent/Object/Process)
5. Assign ETSI domains (1+ domains)
6. Store classification decisions in memory

**Output Format** (stored in memory):
```json
{
  "filename": "Game Engine.md",
  "physicality": "VirtualEntity",
  "role": "Object",
  "inferred_class": "mv:VirtualObject",
  "domains": ["InfrastructureDomain", "CreativeMediaDomain"],
  "rationale": "Software framework - purely digital (Virtual), passive tool (Object)"
}
```

**Memory Keys**: `swarm/classifications/{filename}`

**Coordination Hooks**:
```bash
# Before starting batch
npx claude-flow@alpha hooks pre-task --description "Classify batch N (files 1-15)"

# After each file classified
npx claude-flow@alpha hooks post-edit --file "{filename}" --memory-key "swarm/classifications/{filename}"

# After batch complete
npx claude-flow@alpha hooks notify --message "Batch N classification complete"
npx claude-flow@alpha hooks post-task --task-id "classify-batch-N"
```

**Common Patterns**:
- **Software/Tools** → VirtualObject
- **3D Assets/Models** → VirtualObject
- **Algorithms** → VirtualProcess
- **User Representations** → VirtualAgent
- **Hardware** → PhysicalObject
- **Digital Twins** → HybridObject

---

### 2. Formatter Agent (2 instances)

**Type**: Coder
**Primary Responsibility**: Convert files to standardized Logseq + OWL format
**Batch Size**: 15 files
**Cycle Time**: 2 hours

**Key Tasks**:
1. Read classification decisions from memory
2. Apply TEMPLATE.md structure verbatim
3. Generate section IDs (kebab-case: `game-engine-ontology`)
4. Convert concept names to IRI format (camelCase: `mv:GameEngine`)
5. Extract and structure relationships
6. Assign unique term-ids from coordinator registry
7. Format human-readable About sections
8. Write formatted file to disk

**Naming Conventions**:
| Input | Output Type | Convention | Example |
|-------|-------------|------------|---------|
| `Game Engine.md` | IRI Class | camelCase | `mv:GameEngine` |
| `Game Engine` | Section ID | kebab-case | `game-engine-ontology` |
| `3D Model` | IRI Class | Handle numbers | `mv:ThreeDModel` |
| relationships | Properties | kebab-case | `has-part::`, `requires::` |

**Memory Keys**:
- Read: `swarm/classifications/{filename}`
- Write: `swarm/formatted/{filename}` (status marker)

**Coordination Hooks**:
```bash
# Load classification + term-id registry
npx claude-flow@alpha hooks session-restore --session-id "swarm-migration"
npx claude-flow@alpha hooks pre-task --description "Format batch N"

# After formatting each file
npx claude-flow@alpha hooks post-edit --file "{filename}" --memory-key "swarm/formatted/{filename}"

# After batch complete
npx claude-flow@alpha hooks notify --message "Batch N formatting complete"
npx claude-flow@alpha hooks post-task --task-id "format-batch-N"
```

---

### 3. OWL Agent (2 instances)

**Type**: Specialist
**Primary Responsibility**: Create formal OWL axioms based on classification
**Batch Size**: 15 files
**Cycle Time**: 1.5 hours

**Key Tasks**:
1. Read formatted files from disk
2. Generate minimum required axioms (Declaration + 2 SubClassOf)
3. Add cardinality constraints where appropriate
4. Create domain and layer classifications
5. Validate OWL functional syntax
6. Add property declarations for complex relationships
7. Write axiom blocks to files

**Minimum Axiom Template**:
```clojure
Declaration(Class(mv:[ClassName]))

# Classification - REQUIRED
SubClassOf(mv:[ClassName] mv:[Physicality]Entity)
SubClassOf(mv:[ClassName] mv:[Role])
```

**Advanced Patterns**:
```clojure
# Cardinality - Avatar must represent exactly one agent
SubClassOf(mv:Avatar
  ObjectExactCardinality(1 mv:represents mv:Agent)
)

# Existential - Digital Twin must synchronize with physical entity
SubClassOf(mv:DigitalTwin
  ObjectSomeValuesFrom(mv:synchronizesWith mv:PhysicalEntity)
)

# Property chains - Transitivity patterns
SubObjectPropertyOf(
  ObjectPropertyChain(mv:hasPart mv:hasPart)
  mv:hasPart
)
```

**Memory Keys**:
- Read: `swarm/templates/owl-patterns` (reusable axiom templates)
- Write: `swarm/owl-complete/{filename}` (status marker)

**Coordination Hooks**:
```bash
# Load axiom templates
npx claude-flow@alpha hooks pre-task --description "Generate OWL axioms for batch N"

# After each file axiomatized
npx claude-flow@alpha hooks post-edit --file "{filename}" --memory-key "swarm/owl-complete/{filename}"

# After batch complete
npx claude-flow@alpha hooks notify --message "Batch N OWL axioms complete"
npx claude-flow@alpha hooks post-task --task-id "owl-batch-N"
```

---

### 4. Validator Agent (1 instance)

**Type**: Tester
**Primary Responsibility**: Run extractor tool and verify compliance
**Batch Size**: 30 files (validates 2 batches in parallel)
**Cycle Time**: 30 minutes

**Key Tasks**:
1. Run logseq-owl-extractor on batch files
2. Parse and report validation errors
3. Check syntax correctness
4. Verify required properties present
5. Validate OWL axiom syntax
6. Test inferred class consistency
7. Generate validation reports
8. Flag files for revision if needed

**Validation Levels**:

**Level 1: Individual File**
```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input "VisioningLab/Game Engine.md" \
  --output test-game-engine.ofn \
  --validate
```

**Level 2: Batch**
```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input VisioningLab \
  --filter "batch-N-files.txt" \
  --output batch-N.ofn \
  --validate
```

**Level 3: Full Ontology** (weekly)
```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology-full.ofn \
  --validate
```

**Quality Checklist**:
- [ ] All required properties present (metaverseOntology, term-id, owl:class, physicality, role)
- [ ] OWL axioms parse without errors
- [ ] Section IDs unique and well-formed
- [ ] Inferred class matches physicality + role
- [ ] At least one domain assigned
- [ ] WikiLinks use [[Double Brackets]]
- [ ] Code fence uses ```clojure
- [ ] No extractor warnings

**Memory Keys**:
- Write: `swarm/validation/batch-{N}` (validation report)

**Coordination Hooks**:
```bash
# Before validation
npx claude-flow@alpha hooks pre-task --description "Validate batches N and M"

# After validation
npx claude-flow@alpha hooks post-task --task-id "validate-batches-N-M"
npx claude-flow@alpha hooks notify --message "Validation complete: 28/30 passed, 2 need revision"
```

---

### 5. Coordinator Agent (1 instance)

**Type**: Coordinator
**Primary Responsibility**: Manage workflow, track progress, maintain registries
**Batch Size**: N/A
**Continuous Operation**: Yes

**Key Responsibilities**:
1. Maintain term-id registry (20100+)
2. Track file assignments to agents
3. Monitor batch progress
4. Update MIGRATION_STATUS.md
5. Resolve classification ambiguities
6. Handle duplicate concept detection
7. Coordinate batch assignments
8. Generate daily progress reports
9. Enforce quality gate compliance

**State Management** (Memory Keys):
- `swarm/registry/term-ids` - Master term-id assignments
- `swarm/assignments/active` - Current batch assignments
- `swarm/batches/status` - Batch completion status
- `swarm/validation/summary` - Aggregated validation results
- `swarm/decisions/ambiguous` - Escalated classification decisions
- `swarm/progress/daily` - Daily throughput metrics

**Coordination Hooks**:
```bash
# Session management
npx claude-flow@alpha hooks session-restore --session-id "swarm-migration"
npx claude-flow@alpha hooks session-end --export-metrics true

# Memory operations for all state
npx claude-flow@alpha hooks post-edit --memory-key "swarm/registry/term-ids"
```

**Daily Report Template**:
```markdown
## Daily Progress - 2025-10-14

### Batches Completed Today
- Batch 5 (15 files) - VirtualObjects - ✅ Validated
- Batch 6 (15 files) - VirtualObjects - ✅ Validated

### Statistics
- Files completed today: 30
- Files remaining: 241
- Total completed: 33/274 (12.0%)
- Validation pass rate: 100%

### Issues & Resolutions
- None

### Next Batches Assigned
- Batch 7: Classifier A (PhysicalObjects)
- Batch 8: Classifier B (PhysicalObjects)
```

---

## Workflow Pipeline Design

### Phase 1: Initialization (1 day)

**Coordinator Tasks**:
1. Initialize term-id registry starting at 20100
2. Create file inventory (274 total, 3 complete, 271 remaining)
3. Initialize MIGRATION_STATUS.md with batch structure
4. Set up memory namespaces in MCP

**All Agent Tasks**:
1. Review TEMPLATE.md
2. Study 3 exemplar files (Avatar.md, DigitalTwin.md, Game Engine.md)
3. Read guides/MIGRATION_GUIDE.md
4. Review URIMapping.md conventions

**Validator Tasks**:
1. Build logseq-owl-extractor:
   ```bash
   cd logseq-owl-extractor
   cargo build --release
   ```

---

### Phase 2: Parallel Pipeline Processing (Weeks 1-6)

**Cycle Parameters**:
- Batch size: 15 files
- Parallel batches: 2 (A and B streams)
- Cycle time: 4-6 hours
- Cycles per day: 2
- Daily throughput: 60 files

**Single Cycle Workflow**:

```
┌─────────────────────────────────────────────────────────────────┐
│ Step 1: Coordinator Assigns Batches (5 min)                    │
│  - Batch 5A: Files 61-75 → Classifier A                        │
│  - Batch 5B: Files 76-90 → Classifier B                        │
│  - Update memory: swarm/assignments/active                      │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2: PARALLEL Classification (2 hours)                      │
│  ┌──────────────────────────┐  ┌──────────────────────────┐   │
│  │ Classifier A             │  │ Classifier B             │   │
│  │ - Analyze 15 files       │  │ - Analyze 15 files       │   │
│  │ - Apply decision trees   │  │ - Apply decision trees   │   │
│  │ - Store in memory        │  │ - Store in memory        │   │
│  │   swarm/classifications/ │  │   swarm/classifications/ │   │
│  └──────────────────────────┘  └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 3: PARALLEL Formatting (2 hours)                          │
│  ┌──────────────────────────┐  ┌──────────────────────────┐   │
│  │ Formatter A              │  │ Formatter B              │   │
│  │ - Read classifications   │  │ - Read classifications   │   │
│  │ - Apply TEMPLATE.md      │  │ - Apply TEMPLATE.md      │   │
│  │ - Assign term-ids        │  │ - Assign term-ids        │   │
│  │ - Write formatted files  │  │ - Write formatted files  │   │
│  └──────────────────────────┘  └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 4: PARALLEL OWL Axiom Generation (1.5 hours)              │
│  ┌──────────────────────────┐  ┌──────────────────────────┐   │
│  │ OWL Agent A              │  │ OWL Agent B              │   │
│  │ - Read formatted files   │  │ - Read formatted files   │   │
│  │ - Generate axioms        │  │ - Generate axioms        │   │
│  │ - Add constraints        │  │ - Add constraints        │   │
│  │ - Write to files         │  │ - Write to files         │   │
│  └──────────────────────────┘  └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 5: SEQUENTIAL Validation (30 min)                         │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ Validator Agent                                        │    │
│  │ - Extract Batch A (15 files)                          │    │
│  │ - Extract Batch B (15 files)                          │    │
│  │ - Generate validation reports                         │    │
│  │ - Store results in swarm/validation/batch-{N}         │    │
│  └────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 6: Coordinator Review & Next Assignment (30 min)          │
│  - Review validation reports                                   │
│  - Update MIGRATION_STATUS.md                                  │
│  - Handle any validation failures (assign for rework)          │
│  - Assign next 2 batches (Batch 6A, 6B)                        │
│  - Generate progress report                                    │
└─────────────────────────────────────────────────────────────────┘
```

**Throughput Calculation**:
- Files per cycle: 30 (2 batches × 15 files)
- Cycle time: 5 hours
- Cycles per day: 2
- **Daily throughput: 60 files**
- **Weekly throughput: 300 files**
- Total remaining: 271 files
- **Estimated duration: 4.5 weeks of processing + 1.5 weeks buffer = 6 weeks**

---

### Phase 3: Prioritized File Progression

**Week 1-2: Simple VirtualObjects (120 files)**

Target categories:
- Software tools (Unity, Unreal, Blender, etc.)
- 3D modeling tools
- File formats (glTF, FBX, USD, etc.)
- Game engines and rendering engines
- Development frameworks

Classification: Almost all → `VirtualObject`

**Rationale**: Build confidence with straightforward classifications, establish patterns, minimal complexity.

---

**Week 3-4: PhysicalObjects (80 files)**

Target categories:
- VR/AR headsets (Meta Quest, HoloLens, etc.)
- Sensors and IoT devices
- Motion capture systems
- Haptic devices
- Servers and infrastructure hardware
- Networking equipment

Classification: Almost all → `PhysicalObject`

**Rationale**: Clear physical classification, well-defined components and relationships.

---

**Week 5: VirtualProcesses (40 files)**

Target categories:
- Rendering techniques (ray tracing, rasterization, etc.)
- Authentication protocols
- Encryption algorithms
- Data synchronization processes
- Pathfinding algorithms
- Optimization techniques

Classification: Almost all → `VirtualProcess`

**Rationale**: Requires distinguishing Process from Object, builds classification sophistication.

---

**Week 6: Agents & Hybrids (31 files)**

Target categories:
- AI agents and bots
- NPCs (Non-Player Characters)
- Digital twins
- AR overlay systems
- Mixed reality interfaces
- Teleoperated systems

Classifications: Mix of `VirtualAgent`, `HybridAgent`, `HybridObject`, `HybridProcess`

**Rationale**: Most complex classifications requiring careful agency and binding analysis.

---

### Phase 4: Weekly Quality Assurance

**Frequency**: End of each week

**Tasks**:
1. Run full ontology extraction on all completed files
   ```bash
   ./logseq-owl-extractor/target/release/logseq-owl-extractor \
     --input . \
     --output metaverse-ontology-weekly.ofn \
     --validate
   ```

2. Peer review random 10% sample (12 files/week)
   - Different agent reviews another agent's work
   - Check classification correctness
   - Verify format compliance
   - Validate axiom quality

3. Check classification consistency
   - Ensure similar concepts classified identically
   - Review any ambiguous decisions
   - Update classification decision trees

4. Validate cross-references and relationships
   - Ensure WikiLinks point to existing concepts
   - Check relationship coherence

5. Update memory with learnings
   - Store common patterns in `swarm/templates/owl-patterns`
   - Document edge cases in `swarm/decisions/ambiguous`

---

### Phase 5: Completion (3 days)

**Day 1: Final Validation**
- Run complete ontology extraction
- Fix any remaining validation errors
- Ensure 100% extraction success rate

**Day 2: Documentation**
- Complete MIGRATION_STATUS.md
- Generate final migration report
- Create exemplar showcase document
- Document lessons learned

**Day 3: Handoff**
- Export full ontology to OWL 2 DL
- Run reasoner validation (if available)
- Create usage documentation
- Archive migration artifacts

---

## Coordination Protocols

### MCP Memory Keys Architecture

```
swarm/
├── classifications/
│   ├── Game Engine.md.json       # Classification decision per file
│   ├── Avatar.md.json
│   └── ...
├── registry/
│   └── term-ids.json              # Master term-id assignments
├── assignments/
│   ├── active.json                # Current batch assignments
│   └── batch-{N}.json             # Historical batch records
├── validation/
│   ├── batch-{N}.json             # Validation results per batch
│   └── summary.json               # Aggregated validation stats
├── progress/
│   ├── daily.json                 # Daily completion counts
│   └── weekly.json                # Weekly milestone tracking
├── decisions/
│   └── ambiguous.json             # Escalated classification decisions
└── templates/
    └── owl-patterns.json          # Reusable axiom templates
```

### Hooks Integration Patterns

**Pre-Task Hook** (Before starting work):
```bash
npx claude-flow@alpha hooks pre-task \
  --description "Classify batch 5 (15 VirtualObject files)"
```
Actions: Auto-assign agent by file type, validate commands, prepare resources

**Post-Edit Hook** (After each file):
```bash
npx claude-flow@alpha hooks post-edit \
  --file "VisioningLab/Game Engine.md" \
  --memory-key "swarm/classifications/Game Engine.md"
```
Actions: Auto-format code, train neural patterns, update memory

**Post-Task Hook** (After batch complete):
```bash
npx claude-flow@alpha hooks post-task \
  --task-id "classify-batch-5"
```
Actions: Analyze performance, track token usage, update metrics

**Session Restore Hook** (Resume work):
```bash
npx claude-flow@alpha hooks session-restore \
  --session-id "swarm-migration"
```
Actions: Load previous context, restore agent states, sync memory

**Session End Hook** (End of day):
```bash
npx claude-flow@alpha hooks session-end \
  --export-metrics true
```
Actions: Generate summary, persist state, export daily report

**Notification Hook** (Inter-agent signaling):
```bash
npx claude-flow@alpha hooks notify \
  --message "Batch 5 classification complete, ready for formatting"
```
Actions: Log event, update coordinator dashboard

---

### Communication Patterns

**Async Memory Bus**:
- Agents never directly communicate
- All data exchange via memory keys
- Decoupled, scalable, fault-tolerant

**Batch Handoff Flow**:
```
Classifier → Memory[classifications/file.json]
               ↓
Formatter ← Memory[classifications/file.json]
Formatter → Disk[formatted file] + Memory[formatted/file.json]
               ↓
OWL Agent ← Disk[formatted file]
OWL Agent → Disk[file with axioms] + Memory[owl-complete/file.json]
               ↓
Validator ← Disk[complete files]
Validator → Memory[validation/batch-N.json]
               ↓
Coordinator ← Memory[validation/batch-N.json]
```

**Error Escalation**:
1. Validation failure detected by Validator
2. Stored in `swarm/validation/batch-{N}.json` with error details
3. Coordinator reads validation results
4. Assigns revision task to appropriate agent
5. Updated file re-validated before marking complete

---

## Quality Gates

### Gate 1: Classification Quality

**Enforced By**: Classifier agents (self-check) + Coordinator (audit)

**Criteria**:
- [ ] Physicality dimension assigned (Physical/Virtual/Hybrid)
- [ ] Role dimension assigned (Agent/Object/Process)
- [ ] Inferred class computed correctly (e.g., Virtual + Object = VirtualObject)
- [ ] At least one ETSI domain assigned
- [ ] Rationale documented for non-obvious classifications

**Pass Condition**: All criteria met, stored in memory

**Failure Action**: Flag for coordinator review before proceeding

---

### Gate 2: Format Compliance

**Enforced By**: Formatter agents + Validator (syntax check)

**Criteria**:
- [ ] TEMPLATE.md structure followed exactly
- [ ] All section IDs present (ontology, relationships, owl-axioms, about)
- [ ] Section IDs use kebab-case convention
- [ ] Unique term-id assigned from registry
- [ ] WikiLinks use [[Double Brackets]] format
- [ ] IRI naming: kebab-case → camelCase (Game Engine → mv:GameEngine)
- [ ] Properties use kebab-case (has-part, requires, enables)

**Pass Condition**: File structure matches template precisely

**Failure Action**: Automatic rework by formatter agent

---

### Gate 3: OWL Axiom Quality

**Enforced By**: OWL agents + Validator (syntax validation)

**Criteria**:
- [ ] Minimum 3 axioms present:
  - `Declaration(Class(mv:[ClassName]))`
  - `SubClassOf(mv:[ClassName] mv:[Physicality]Entity)`
  - `SubClassOf(mv:[ClassName] mv:[Role])`
- [ ] OWL functional syntax is valid
- [ ] Axioms consistent with classification
- [ ] Code block uses ```clojure syntax highlighting
- [ ] Property declarations included for custom properties
- [ ] Cardinality constraints where appropriate

**Pass Condition**: Axioms parse and align with classification

**Failure Action**: OWL agent revises axioms

---

### Gate 4: Extraction Success

**Enforced By**: Validator agent

**Criteria**:
- [ ] File extracts without errors via logseq-owl-extractor
- [ ] All required properties present in extraction
- [ ] No syntax warnings
- [ ] Inferred class matches declared classification
- [ ] Integration test passes (works with full ontology)

**Pass Condition**: Clean extraction with zero errors

**Failure Action**: Return to appropriate agent for fix (formatter or OWL)

---

## Risk Mitigation

### Risk 1: Term-ID Collision

**Probability**: Medium
**Impact**: Low

**Scenario**: Two formatter agents assign same term-id simultaneously

**Mitigation**:
1. Coordinator maintains authoritative registry in memory
2. Formatter agents request term-id from coordinator before assignment
3. If collision detected, second agent immediately updates their file
4. Memory transactions ensure atomic updates

**Detection**: Coordinator audits term-id registry after each batch

---

### Risk 2: Classification Inconsistency

**Probability**: Medium
**Impact**: High

**Scenario**: Similar concepts classified differently by different agents

**Example**: "Rendering Engine" classified as VirtualObject by Agent A, but "Physics Engine" classified as VirtualProcess by Agent B

**Mitigation**:
1. Weekly consistency review comparing similar concepts
2. Maintain classification decision log in `swarm/decisions/ambiguous`
3. Peer review 10% random sample weekly
4. Update guides/MIGRATION_GUIDE.md with new patterns
5. Coordinator resolves ambiguous cases

**Detection**: Pattern analysis during QA phase, cross-referencing similar terms

---

### Risk 3: Validation Failures

**Probability**: Low
**Impact**: Medium

**Scenario**: Batch fails extraction due to syntax errors

**Mitigation**:
1. Self-validation by each agent before handoff
2. Incremental validation (file → batch → full ontology)
3. Validator catches issues early in cycle (before next batch starts)
4. Automated syntax checking in OWL agent
5. Template-based generation reduces manual errors

**Detection**: Validator reports failures immediately after each batch

---

### Risk 4: Agent Availability

**Probability**: Medium
**Impact**: Medium

**Scenario**: One agent instance unavailable, blocking pipeline

**Mitigation**:
1. 2 instances of each agent type (except coordinator/validator)
2. Work continues with remaining instances at reduced throughput
3. Memory-based coordination allows resume without context loss
4. Coordinator can reassign batches to available agents

**Detection**: Monitoring batch completion times, coordinator dashboard

---

### Risk 5: Scope Creep

**Probability**: Low
**Impact**: High

**Scenario**: Format changes mid-migration, requiring rework

**Mitigation**:
1. Strict adherence to TEMPLATE.md with no deviations
2. Coordinator enforces quality gates rigorously
3. Format changes documented but deferred to post-migration
4. Lock TEMPLATE.md with version control
5. Any clarifications added to guides, not structure changes

**Detection**: Code review of TEMPLATE.md changes

---

## Success Metrics

### Quantitative Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Completion Rate** | 271/271 files (100%) | Count in MIGRATION_STATUS.md |
| **Validation Pass Rate** | 100% (no extraction errors) | Extractor success rate |
| **Average Time per File** | 20 minutes | Total time / files completed |
| **Daily Throughput** | 60 files/day | Files completed per day |
| **Defect Escape Rate** | <5% requiring rework | Rework count / total files |
| **Classification Consistency** | 95%+ for similar concepts | Manual audit sample |

### Qualitative Metrics

| Metric | Evaluation Method |
|--------|-------------------|
| **Classification Consistency** | Similar concepts classified identically (manual review) |
| **Definition Clarity** | Clear, precise, one-sentence definitions (peer review) |
| **Axiom Quality** | Axioms capture essential constraints (OWL expert review) |
| **Documentation Completeness** | About sections informative and well-structured (readability) |
| **Ontology Coherence** | Full ontology extracts and reasons without errors (reasoner test) |

---

## Timeline & Milestones

```
Week 0 (Day 1): Initialization
├── Coordinator: Setup registries, create file inventory
├── All agents: Review reference materials
└── Validator: Build extractor tool

Week 1: Simple VirtualObjects (Phase 1)
├── Day 1: 60 files (Batches 1-2)
├── Day 2: 60 files (Batches 3-4)
└── Weekly QA: Full extraction + 12 file peer review
Target: 120 files completed (120/271 = 44%)

Week 2: Continue VirtualObjects + Start PhysicalObjects
├── Day 1: 60 files (Batches 5-6)
├── Day 2: 60 files (Batches 7-8)
└── Weekly QA
Target: 240 files total (240/271 = 89%)

Week 3: PhysicalObjects + VirtualProcesses
├── Day 1: 31 remaining VirtualObjects + PhysicalObjects
├── Day 2: Continue PhysicalObjects
└── Weekly QA
Target: All VirtualObjects + PhysicalObjects complete

Week 4: VirtualProcesses + Agents/Hybrids
├── Day 1: VirtualProcesses
├── Day 2: Complex agents and hybrids
└── Weekly QA
Target: All VirtualProcesses complete

Week 5: Final Agents/Hybrids + Edge Cases
├── Day 1-2: Remaining complex cases
└── Weekly QA + Final validation
Target: 100% files complete

Week 6: Final QA & Documentation
├── Day 1: Full ontology validation
├── Day 2: Documentation and reports
├── Day 3: Handoff and archival
Target: Project complete, all artifacts delivered
```

**Estimated Completion Date**: 2025-11-25

---

## Usage Instructions

### For Coordinator

1. **Initialize swarm**:
   ```bash
   npx claude-flow@alpha swarm init --topology mesh --maxAgents 8
   npx claude-flow@alpha memory store --key "swarm/registry/term-ids" --value '{"next": 20100, "assigned": []}'
   ```

2. **Assign batches**:
   ```bash
   # Update memory with batch assignments
   npx claude-flow@alpha memory store \
     --key "swarm/assignments/batch-1" \
     --value '{"agent": "classifier-a", "files": ["file1.md", "file2.md", ...], "status": "assigned"}'
   ```

3. **Monitor progress**:
   ```bash
   # Check memory for status
   npx claude-flow@alpha memory retrieve --key "swarm/progress/daily"
   npx claude-flow@alpha swarm status
   ```

### For Classifier Agents

1. **Claim batch**:
   ```bash
   npx claude-flow@alpha hooks pre-task --description "Classify batch 1"
   ```

2. **Classify files** (read from VisioningLab/, apply decision trees)

3. **Store classifications**:
   ```bash
   npx claude-flow@alpha hooks post-edit \
     --file "VisioningLab/Game Engine.md" \
     --memory-key "swarm/classifications/Game Engine.md"
   ```

4. **Complete batch**:
   ```bash
   npx claude-flow@alpha hooks post-task --task-id "classify-batch-1"
   npx claude-flow@alpha hooks notify --message "Batch 1 classification complete"
   ```

### For Formatter Agents

1. **Retrieve classifications**:
   ```bash
   npx claude-flow@alpha hooks session-restore --session-id "swarm-migration"
   npx claude-flow@alpha memory retrieve --key "swarm/classifications/Game Engine.md"
   ```

2. **Format files** (apply TEMPLATE.md structure)

3. **Request term-id** from coordinator registry

4. **Write formatted file** and update memory

### For OWL Agents

1. **Read formatted files** from disk

2. **Generate axioms** following minimum template + advanced patterns

3. **Write axioms** to file

4. **Signal completion**:
   ```bash
   npx claude-flow@alpha hooks post-edit --file "{filename}" --memory-key "swarm/owl-complete/{filename}"
   ```

### For Validator

1. **Build extractor** (one-time):
   ```bash
   cd logseq-owl-extractor
   cargo build --release
   ```

2. **Validate batch**:
   ```bash
   ./target/release/logseq-owl-extractor \
     --input VisioningLab \
     --output batch-1.ofn \
     --validate
   ```

3. **Store results**:
   ```bash
   npx claude-flow@alpha memory store \
     --key "swarm/validation/batch-1" \
     --value '{"passed": 14, "failed": 1, "errors": [...]}'
   ```

---

## Conclusion

This architecture provides a robust, scalable, and quality-focused approach to migrating 271 ontology files. The parallel processing strategy achieves 60 files/day throughput while maintaining 100% validation success through rigorous quality gates. Memory-based coordination via MCP enables loose coupling and fault tolerance, while claude-flow hooks automate repetitive tasks and track progress.

**Expected Outcomes**:
- 6 weeks to completion
- 100% extraction success rate
- Consistent classification across similar concepts
- High-quality OWL axioms capturing essential constraints
- Comprehensive documentation for future maintenance

**Next Steps**:
1. Review and approve this architecture
2. Initialize swarm and registries (Week 0, Day 1)
3. Begin Phase 1 with simple VirtualObjects (Week 1)
4. Monitor daily progress and adjust as needed

---

**Design Document Version**: 1.0
**Stored in Memory**: `swarm/architecture/workflow-design`
**Review Date**: 2025-10-14
**Approved By**: [Pending]
