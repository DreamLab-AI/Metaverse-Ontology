# Agent Spawn Instructions - Batch 1.1

**Orchestration Session**: 2025-10-14
**Swarm ID**: swarm_1760452204542_l7ndgnllo
**Topology**: Mesh (peer-to-peer)
**Batch**: 1.1 (9 files, term-ids 20100-20108)

---

## 🚀 Concurrent Agent Spawn Commands

**CRITICAL**: All 4 agents MUST be spawned in a SINGLE message using Claude Code's Task tool for maximum parallelism.

---

## Agent 1: Classifier Agent

**Type**: `analyst`
**Files**: 3 (API Standard, Authoring Tool, Cloud Rendering Service)
**Term-IDs**: 20100-20102

### Task Instructions

```markdown
You are the **Classifier Agent** for VisioningLab Batch 1.1 migration.

**Your Mission**: Analyze 3 concept files and determine their orthogonal classification (Physicality × Role).

**Files to Classify**:
1. `/home/devuser/workspace/OntologyDesign/VisioningLab/API Standard.md` → term-id: 20100
2. `/home/devuser/workspace/OntologyDesign/VisioningLab/Authoring Tool.md` → term-id: 20101
3. `/home/devuser/workspace/OntologyDesign/VisioningLab/Cloud Rendering Service.md` → term-id: 20102

**Classification Framework**:

**Dimension 1: Physicality**
- PhysicalEntity: Has physical form (hardware, sensors, devices)
- VirtualEntity: Purely digital (software, data, protocols)
- HybridEntity: Binds physical + virtual (digital twin, AR overlay)

**Dimension 2: Role**
- Agent: Autonomous decision-maker with agency
- Object: Passive entity that can be acted upon
- Process: Activity, transformation, or operation

**Expected Classifications**: All 3 files should be VirtualObject (VirtualEntity + Object)

**Primary ETSI Domains**:
- InfrastructureDomain (APIs, services, middleware)
- CreativeMediaDomain (authoring, rendering, content)

**For Each File**:
1. Read original content from VisioningLab/
2. Analyze concept semantics
3. Determine Physicality dimension with rationale
4. Determine Role dimension with rationale
5. Identify primary ETSI domain(s)
6. Store classification in memory

**Memory Storage**:
Use MCP memory to store your analysis:

```bash
# Store classification for each file
npx claude-flow@alpha memory store \
  --key "swarm/classifier/api-standard" \
  --value '{"physicality": "VirtualEntity", "role": "Object", "inferred_class": "VirtualObject", "domain": "InfrastructureDomain", "rationale": "API Standard is purely digital specification with no physical form or agency"}'

npx claude-flow@alpha memory store \
  --key "swarm/classifier/authoring-tool" \
  --value '{"physicality": "VirtualEntity", "role": "Object", "inferred_class": "VirtualObject", "domain": "CreativeMediaDomain", "rationale": "Authoring Tool is software with no physical form or autonomous agency"}'

npx claude-flow@alpha memory store \
  --key "swarm/classifier/cloud-rendering" \
  --value '{"physicality": "VirtualEntity", "role": "Object", "inferred_class": "VirtualObject", "domain": "CreativeMediaDomain", "rationale": "Cloud Rendering Service is virtual service with no physical form"}'
```

**Coordination Hooks**:
```bash
# Before starting
npx claude-flow@alpha hooks pre-task --description "Classify Batch 1.1 files 1-3"

# After completing each file
npx claude-flow@alpha hooks post-edit --file "API Standard.md" --memory-key "swarm/classifier/api-standard"

# After completing all files
npx claude-flow@alpha hooks post-task --task-id "classifier-batch-1.1"
```

**Deliverables**:
1. Classification analysis for 3 files stored in memory
2. Rationale documentation for each classification
3. Domain assignments
4. Status update to coordinator

**Reference Documents**:
- Classification guide: `/home/devuser/workspace/OntologyDesign/task.md` (lines 90-130)
- ETSI domains: `/home/devuser/workspace/OntologyDesign/docs/reference/FORMAT_STANDARDIZED.md`
- Exemplars: Avatar.md (VirtualAgent), Game Engine.md (VirtualObject)

**Success Criteria**:
- All 3 files classified correctly
- Memory storage successful
- Rationale clearly documented
- Ready for Formatter Agent to proceed
```

---

## Agent 2: Formatter Agent

**Type**: `coder`
**Files**: 3 (CDN, Generative Design Tool, Middleware)
**Term-IDs**: 20103-20105

### Task Instructions

```markdown
You are the **Formatter Agent** for VisioningLab Batch 1.1 migration.

**Your Mission**: Convert 3 concept files to standardized Logseq+OWL format using TEMPLATE.md structure.

**Files to Format**:
1. `/home/devuser/workspace/OntologyDesign/VisioningLab/Content Delivery Network (CDN).md` → term-id: 20103
2. `/home/devuser/workspace/OntologyDesign/VisioningLab/Generative Design Tool.md` → term-id: 20104
3. `/home/devuser/workspace/OntologyDesign/VisioningLab/Middleware.md` → term-id: 20105

**Template Reference**: `/home/devuser/workspace/OntologyDesign/docs/reference/TEMPLATE.md`

**Exemplars**:
- `/home/devuser/workspace/OntologyDesign/Avatar.md` (VirtualAgent)
- `/home/devuser/workspace/OntologyDesign/VisioningLab/Game Engine.md` (VirtualObject)

**For Each File**:

1. **Read Original Content**:
   ```bash
   cat "/home/devuser/workspace/OntologyDesign/VisioningLab/Content Delivery Network (CDN).md"
   ```

2. **Retrieve Classifier Analysis** (if available):
   ```bash
   npx claude-flow@alpha memory retrieve --key "swarm/classifier/cdn"
   ```

3. **Apply TEMPLATE.md Structure**:
   - Create `### OntologyBlock` with `collapsed:: true`
   - Add `metaverseOntology:: true` as FIRST property
   - Set `term-id::` (20103, 20104, 20105)
   - Set `preferred-term::` from filename
   - Write clear `definition::`
   - Set `maturity:: draft`
   - Set `owl:class:: mv:[ClassName]` (camelCase)
   - Set `owl:physicality:: VirtualEntity`
   - Set `owl:role:: Object`
   - Set `owl:inferred-class:: mv:VirtualObject`
   - Set `belongsToDomain::` (InfrastructureDomain or CreativeMediaDomain)

4. **Format Relationships**:
   - `has-part::` → Components as wikilinks
   - `requires::` → Dependencies as wikilinks
   - `enables::` → Capabilities as wikilinks

5. **Create OWL Axioms Placeholder**:
   ```markdown
   - #### OWL Axioms
     id:: [concept]-owl-axioms
     collapsed:: true
       - ```clojure
         Declaration(Class(mv:[ClassName]))
         SubClassOf(mv:[ClassName] mv:VirtualEntity)
         SubClassOf(mv:[ClassName] mv:Object)
         ```
   ```

6. **Write "About" Section**:
   - Human-readable description
   - Key characteristics (3-5 bullets)
   - Common implementations
   - Standards and references

7. **Save Formatted File**:
   Overwrite original file in VisioningLab/ directory

8. **Store Formatting State**:
   ```bash
   npx claude-flow@alpha memory store \
     --key "swarm/formatter/cdn" \
     --value '{"status": "complete", "term_id": 20103, "file": "Content Delivery Network (CDN).md"}'
   ```

**Naming Conventions**:
- File: `Content Delivery Network (CDN).md` → Class: `mv:ContentDeliveryNetwork`
- Section ID: `content-delivery-network-ontology`
- Use kebab-case for IDs, camelCase for IRIs

**Coordination Hooks**:
```bash
# Before starting
npx claude-flow@alpha hooks pre-task --description "Format Batch 1.1 files 4-6"

# After each file
npx claude-flow@alpha hooks post-edit --file "Content Delivery Network (CDN).md"

# After completion
npx claude-flow@alpha hooks post-task --task-id "formatter-batch-1.1"
```

**Success Criteria**:
- All 3 files converted to standard format
- All required properties present
- Section IDs follow convention
- Files saved to VisioningLab/
- Memory state updated
```

---

## Agent 3: OWL Agent

**Type**: `specialist` (OWL/ontology expert)
**Files**: 2 (Physics Engine, Policy Engine)
**Term-IDs**: 20106-20107

### Task Instructions

```markdown
You are the **OWL Agent** for VisioningLab Batch 1.1 migration.

**Your Mission**: Create formal OWL 2 Functional Syntax axioms for 2 concept files.

**Files to Process**:
1. `/home/devuser/workspace/OntologyDesign/VisioningLab/Physics Engine.md` → term-id: 20106
2. `/home/devuser/workspace/OntologyDesign/VisioningLab/Policy Engine.md` → term-id: 20107

**Expected Classification**: Both are VirtualObject (VirtualEntity + Object)

**For Each File**:

1. **Read File** (should already be formatted by Formatter Agent, or format yourself)

2. **Retrieve Classification**:
   ```bash
   npx claude-flow@alpha memory retrieve --key "swarm/classifier/physics-engine"
   ```

3. **Create Minimum Required Axioms**:
   ```clojure
   Declaration(Class(mv:PhysicsEngine))

   # Physicality dimension
   SubClassOf(mv:PhysicsEngine mv:VirtualEntity)

   # Role dimension
   SubClassOf(mv:PhysicsEngine mv:Object)

   # Domain classification
   SubClassOf(mv:PhysicsEngine
     ObjectSomeValuesFrom(mv:belongsToDomain mv:CreativeMediaDomain)
   )
   ```

4. **Add Domain-Specific Constraints** (if applicable):

   For **Physics Engine**:
   ```clojure
   # Physics Engine must have simulation capabilities
   SubClassOf(mv:PhysicsEngine
     ObjectSomeValuesFrom(mv:hasCapability mv:PhysicsSimulation)
   )

   # Physics Engine requires 3D scene data
   SubClassOf(mv:PhysicsEngine
     ObjectSomeValuesFrom(mv:requires mv:SceneGraph)
   )
   ```

   For **Policy Engine**:
   ```clojure
   # Policy Engine must evaluate rules
   SubClassOf(mv:PolicyEngine
     ObjectSomeValuesFrom(mv:hasCapability mv:RuleEvaluation)
   )

   # Policy Engine requires policy definitions
   SubClassOf(mv:PolicyEngine
     ObjectSomeValuesFrom(mv:requires mv:PolicyDefinition)
   )
   ```

5. **Format in Code Fence**:
   ```markdown
   - #### OWL Axioms
     id:: physics-engine-owl-axioms
     collapsed:: true
       - ```clojure
         Declaration(Class(mv:PhysicsEngine))
         SubClassOf(mv:PhysicsEngine mv:VirtualEntity)
         SubClassOf(mv:PhysicsEngine mv:Object)

         # Domain constraints
         SubClassOf(mv:PhysicsEngine
           ObjectSomeValuesFrom(mv:belongsToDomain mv:CreativeMediaDomain)
         )

         # Capability requirements
         SubClassOf(mv:PhysicsEngine
           ObjectSomeValuesFrom(mv:hasCapability mv:PhysicsSimulation)
         )
         ```
   ```

6. **Validate Syntax**:
   - Check parentheses balance
   - Verify class names use mv: prefix
   - Ensure camelCase naming
   - Validate functional syntax structure

7. **Store Axioms in Memory**:
   ```bash
   npx claude-flow@alpha memory store \
     --key "swarm/owl/physics-engine" \
     --value '{"status": "complete", "axioms_count": 5, "has_constraints": true}'
   ```

**OWL Functional Syntax Guidelines**:
- Use `mv:` prefix for all metaverse ontology classes
- Use camelCase for class names (PhysicsEngine, not Physics_Engine)
- Indent nested expressions for readability
- Add comments with `#` for human understanding
- Keep axioms minimal but meaningful

**Reference**:
- OWL 2 Primer: https://www.w3.org/TR/owl2-primer/
- Exemplar axioms: Avatar.md, DigitalTwin.md

**Coordination Hooks**:
```bash
# Before starting
npx claude-flow@alpha hooks pre-task --description "Create OWL axioms for Batch 1.1 files 7-8"

# After completion
npx claude-flow@alpha hooks post-task --task-id "owl-batch-1.1"
```

**Success Criteria**:
- OWL axioms syntactically valid
- Minimum required axioms present (Declaration + 2x SubClassOf)
- Domain-specific constraints added where appropriate
- Axioms properly formatted in code fence
- Memory state updated
```

---

## Agent 4: Validator Agent

**Type**: `tester`
**Files**: 1 (WebXR API) + validate all previous 8
**Term-ID**: 20108

### Task Instructions

```markdown
You are the **Validator Agent** for VisioningLab Batch 1.1 migration.

**Your Mission**: Complete WebXR API.md migration AND validate all 9 Batch 1.1 files using the extractor tool.

**Dual Role**:
1. Migrate WebXR API.md (term-id: 20108) to standard format
2. Run extractor validation on all 9 files

---

### Part 1: Migrate WebXR API.md

**File**: `/home/devuser/workspace/OntologyDesign/VisioningLab/WebXR API.md`
**Term-ID**: 20108
**Expected Class**: VirtualObject (VirtualEntity + Object)
**Domain**: InfrastructureDomain

**Steps**:
1. Read original file
2. Apply TEMPLATE.md structure (same as Formatter Agent)
3. Classify as VirtualObject
4. Create OWL axioms (same as OWL Agent)
5. Write complete standardized file
6. Save to VisioningLab/

**WebXR API Specific Details**:
- `preferred-term:: WebXR API`
- `definition:: Web standard API enabling immersive XR experiences in web browsers`
- `owl:class:: mv:WebXRAPI`
- `belongsToDomain:: [[InfrastructureDomain]]`
- `source:: [[W3C WebXR Device API]]`

---

### Part 2: Validate All 9 Files

**Files to Validate**:
1. API Standard.md (20100)
2. Authoring Tool.md (20101)
3. Cloud Rendering Service.md (20102)
4. Content Delivery Network (CDN).md (20103)
5. Generative Design Tool.md (20104)
6. Middleware.md (20105)
7. Physics Engine.md (20106)
8. Policy Engine.md (20107)
9. WebXR API.md (20108)

**Validation Process**:

1. **Build Extractor Tool**:
   ```bash
   cd /home/devuser/workspace/OntologyDesign/logseq-owl-extractor
   cargo build --release
   ```

2. **Run Extraction on Batch 1.1**:
   ```bash
   ./target/release/logseq-owl-extractor \
     --input /home/devuser/workspace/OntologyDesign/VisioningLab \
     --output /home/devuser/workspace/OntologyDesign/test-batch-1.1.ofn \
     --validate
   ```

3. **Check for Errors**:
   - Syntax errors in OWL axioms
   - Missing required properties (metaverseOntology, term-id, owl:class, etc.)
   - Incorrect indentation
   - Mismatched classifications (e.g., VirtualAgent but classified as Object)
   - Invalid wikilinks
   - Malformed section IDs

4. **Generate Validation Report**:
   Create `/home/devuser/workspace/OntologyDesign/docs/orchestration/BATCH_1.1_VALIDATION_REPORT.md`:

   ```markdown
   # Batch 1.1 Validation Report

   **Date**: 2025-10-14
   **Files Validated**: 9
   **Extractor Version**: [version]

   ## Summary
   - ✅ Files Passed: [count]
   - ❌ Files Failed: [count]
   - ⚠️ Warnings: [count]

   ## File-by-File Results

   ### API Standard.md (20100)
   - Status: ✅ PASS / ❌ FAIL
   - Errors: [list or "None"]
   - Warnings: [list or "None"]

   [... repeat for all 9 files ...]

   ## Issues Found

   ### Critical Errors
   1. [File]: [Error description]

   ### Warnings
   1. [File]: [Warning description]

   ## Recommendations
   - [Action items for fixes]

   ## OWL Output
   - Output file: test-batch-1.1.ofn
   - Size: [bytes]
   - Classes extracted: [count]
   - Axioms generated: [count]
   ```

5. **Store Validation Results**:
   ```bash
   npx claude-flow@alpha memory store \
     --key "swarm/validator/batch-1.1-report" \
     --value '{"files_validated": 9, "passed": 9, "failed": 0, "warnings": 2, "timestamp": "2025-10-14T12:00:00Z"}'
   ```

6. **Update MIGRATION_CHECKLIST.md**:
   Mark Batch 1.1 files as complete:
   ```markdown
   #### Batch 1.1 - Software & Tools (Priority: Critical)
   **Agent**: Validator Agent | **Status**: ✅ Complete | **Term-IDs**: 20100-20108

   | # | Status | File | Class | Term-ID | Domain | Validated |
   |---|--------|------|-------|---------|--------|-----------|
   | 1 | ✅ | API Standard.md | VirtualObject | 20100 | Infrastructure | ✅ |
   | 2 | ✅ | Authoring Tool.md | VirtualObject | 20101 | CreativeMedia | ✅ |
   [... etc ...]
   ```

**Coordination Hooks**:
```bash
# Before starting
npx claude-flow@alpha hooks pre-task --description "Validate Batch 1.1 all files"

# After migration
npx claude-flow@alpha hooks post-edit --file "WebXR API.md"

# After validation
npx claude-flow@alpha hooks post-task --task-id "validator-batch-1.1"
```

**Success Criteria**:
- WebXR API.md migrated to standard format
- All 9 files extract without critical errors
- Validation report generated
- MIGRATION_CHECKLIST.md updated
- OWL output file created (test-batch-1.1.ofn)
- Memory state shows batch complete
```

---

## 🎯 Orchestrator Monitoring

As the **Task Orchestrator**, you will monitor agent progress via memory:

```bash
# Check overall batch status
npx claude-flow@alpha memory retrieve --key "swarm/batch-1.1/status"

# Check agent progress
npx claude-flow@alpha memory retrieve --key "swarm/classifier/api-standard"
npx claude-flow@alpha memory retrieve --key "swarm/formatter/cdn"
npx claude-flow@alpha memory retrieve --key "swarm/owl/physics-engine"
npx claude-flow@alpha memory retrieve --key "swarm/validator/batch-1.1-report"

# Check for errors
npx claude-flow@alpha memory retrieve --key "swarm/validator/errors"
```

---

## 📊 Success Metrics

Batch 1.1 is COMPLETE when:
- ✅ All 9 files migrated to standard format
- ✅ All files extract without critical errors
- ✅ All term-ids assigned (20100-20108)
- ✅ Validation report shows 100% success or acceptable warnings only
- ✅ MIGRATION_CHECKLIST.md updated
- ✅ Memory state shows all agents complete
- ✅ Test extraction file generated (test-batch-1.1.ofn)

---

## 🚨 Error Handling

If any agent encounters blocking issues:
1. Agent stores error in memory: `swarm/[agent]/errors`
2. Agent notifies coordinator via hooks
3. Coordinator reviews error
4. Coordinator spawns fix agent or reassigns work
5. Batch continues with other files

---

**Ready to Execute**: All instructions prepared for concurrent agent spawn via Claude Code's Task tool.

**Next Step**: Spawn all 4 agents in a SINGLE message using Task tool.
