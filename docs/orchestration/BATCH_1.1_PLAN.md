# Batch 1.1 Migration Plan - Software & Tools

**Coordinator**: Task Orchestrator Agent
**Target**: 9 files (API Standard.md through WebXR API.md)
**Term-IDs**: 20100-20108
**Priority**: CRITICAL
**Estimated Time**: 3-4 hours
**Status**: IN PROGRESS

---

## 🎯 Objectives

Migrate the first batch of simple VirtualObject concepts from VisioningLab to standardized Logseq+OWL format, establishing workflow patterns for the remaining 265 files.

---

## 📋 File Assignments

### Classifier Agent - Files 1-3
**Role**: Analyze concepts and determine Physicality × Role classification

| # | File | Est. Class | Term-ID | Status |
|---|------|------------|---------|--------|
| 1 | API Standard.md | VirtualObject | 20100 | Pending |
| 2 | Authoring Tool.md | VirtualObject | 20101 | Pending |
| 3 | Cloud Rendering Service.md | VirtualObject | 20102 | Pending |

**Tasks**:
- Read original files from VisioningLab/
- Analyze concept semantics
- Determine: VirtualEntity + Object classification
- Identify primary ETSI domain
- Document classification rationale
- Store analysis in memory: `swarm/classifier/batch-1.1/[filename]`

**Memory Keys**:
- `swarm/classifier/api-standard` → Classification data
- `swarm/classifier/authoring-tool` → Classification data
- `swarm/classifier/cloud-rendering` → Classification data

### Formatter Agent - Files 4-6
**Role**: Apply TEMPLATE.md structure and convert to Logseq format

| # | File | Est. Class | Term-ID | Status |
|---|------|------------|---------|--------|
| 4 | Content Delivery Network (CDN).md | VirtualObject | 20103 | Pending |
| 5 | Generative Design Tool.md | VirtualObject | 20104 | Pending |
| 6 | Middleware.md | VirtualObject | 20105 | Pending |

**Tasks**:
- Read original content
- Check classifier's analysis in memory
- Apply TEMPLATE.md structure
- Create OntologyBlock with all required properties
- Format relationships as wikilinks
- Write human-readable "About" section
- Save formatted files back to VisioningLab/
- Store state: `swarm/formatter/batch-1.1/[filename]`

**Memory Keys**:
- `swarm/formatter/cdn` → Formatting state
- `swarm/formatter/generative-design` → Formatting state
- `swarm/formatter/middleware` → Formatting state

### OWL Agent - Files 7-8
**Role**: Create formal OWL 2 axioms based on classification

| # | File | Est. Class | Term-ID | Status |
|---|------|------------|---------|--------|
| 7 | Physics Engine.md | VirtualObject | 20106 | Pending |
| 8 | Policy Engine.md | VirtualObject | 20107 | Pending |

**Tasks**:
- Read formatted files from Formatter Agent
- Check classifier's analysis
- Write OWL Functional Syntax axioms:
  - Declaration(Class(mv:ClassName))
  - SubClassOf(mv:ClassName mv:VirtualEntity)
  - SubClassOf(mv:ClassName mv:Object)
  - Domain/Layer axioms
- Add cardinality constraints where needed
- Validate OWL syntax
- Store axioms: `swarm/owl/batch-1.1/[filename]`

**Memory Keys**:
- `swarm/owl/physics-engine` → OWL axioms
- `swarm/owl/policy-engine` → OWL axioms

### Validator Agent - Files 9
**Role**: Run extractor tool and verify compliance

| # | File | Est. Class | Term-ID | Status |
|---|------|------------|---------|--------|
| 9 | WebXR API.md | VirtualObject | 20108 | Pending |

**Tasks**:
- Read all completed files in Batch 1.1
- Run logseq-owl-extractor on each file
- Check for:
  - Syntax errors in OWL axioms
  - Missing required properties
  - Incorrect indentation
  - Mismatched classifications
  - Invalid wikilinks
- Report issues to memory: `swarm/validator/batch-1.1/errors`
- Generate validation report
- Mark compliant files in MIGRATION_CHECKLIST.md

**Memory Keys**:
- `swarm/validator/batch-1.1-report` → Validation results
- `swarm/validator/errors` → Error list for fixes

---

## 🔄 Workflow Pattern

```
┌─────────────────┐
│ Classifier      │ → Analyzes 3 files → Stores classification
│ Agent (1-3)     │                      in memory
└─────────────────┘
         ↓
┌─────────────────┐
│ Formatter       │ → Formats 3 files  → Retrieves classifier
│ Agent (4-6)     │                      data from memory
└─────────────────┘
         ↓
┌─────────────────┐
│ OWL Agent       │ → Creates axioms   → Uses formatter +
│ (7-8)           │   for 2 files        classifier data
└─────────────────┘
         ↓
┌─────────────────┐
│ Validator       │ → Validates 1 file → Runs extractor,
│ Agent (9)       │   + prior work       reports issues
└─────────────────┘
         ↓
┌─────────────────┐
│ Coordinator     │ → Updates status   → Marks batch
│ (Orchestrator)  │                      complete
└─────────────────┘
```

---

## 🧠 Memory Coordination

All agents use MCP memory for coordination:

**Namespace**: `swarm/batch-1.1/`

**Structure**:
```
swarm/
├── batch-1.1/
│   ├── status → "in_progress"
│   ├── coordinator → "task-orchestrator-001"
│   ├── start-time → "2025-10-14T10:00:00Z"
│   ├── classifier/
│   │   ├── api-standard → {physicality: "Virtual", role: "Object"}
│   │   ├── authoring-tool → {physicality: "Virtual", role: "Object"}
│   │   └── cloud-rendering → {physicality: "Virtual", role: "Object"}
│   ├── formatter/
│   │   ├── cdn → {status: "complete", term-id: 20103}
│   │   ├── generative-design → {status: "complete", term-id: 20104}
│   │   └── middleware → {status: "complete", term-id: 20105}
│   ├── owl/
│   │   ├── physics-engine → {axioms: "...", status: "complete"}
│   │   └── policy-engine → {axioms: "...", status: "complete"}
│   └── validator/
│       ├── report → {files_validated: 9, errors: 0, warnings: 2}
│       └── errors → []
```

---

## 📊 Success Metrics

**Required for Batch Completion**:
- ✅ All 9 files migrated to standard format
- ✅ All files extract without errors
- ✅ All term-ids assigned (20100-20108)
- ✅ All classifications validated
- ✅ MIGRATION_CHECKLIST.md updated
- ✅ Validation report generated

**Quality Criteria**:
- 100% extraction success rate
- 0 critical errors
- <5 warnings acceptable
- Average time per file: <20 minutes

---

## 🚨 Risk Mitigation

**Risk**: Agent fails or produces invalid output
**Mitigation**: Validator Agent catches errors, requests rework

**Risk**: Classification ambiguity
**Mitigation**: Classifier Agent documents rationale, escalates to coordinator

**Risk**: term-id collision
**Mitigation**: Sequential assignment, memory tracking

**Risk**: Extractor validation fails
**Mitigation**: OWL Agent validates syntax before committing

---

## 📝 Agent Instructions

### Classifier Agent Prompt:
```
Analyze these 3 VisioningLab files and determine classification:

1. API Standard.md
2. Authoring Tool.md
3. Cloud Rendering Service.md

For each file:
- Read original content
- Determine Physicality (Physical/Virtual/Hybrid)
- Determine Role (Agent/Object/Process)
- Identify primary ETSI domain
- Document rationale

Store analysis in memory:
- mcp__claude-flow__memory_usage {action: "store", key: "swarm/classifier/[file]", value: {...}}

Expected: All are VirtualObject (VirtualEntity + Object)
```

### Formatter Agent Prompt:
```
Format these 3 VisioningLab files to TEMPLATE.md structure:

4. Content Delivery Network (CDN).md → term-id: 20103
5. Generative Design Tool.md → term-id: 20104
6. Middleware.md → term-id: 20105

For each file:
- Read classifier analysis from memory
- Apply TEMPLATE.md structure
- Create OntologyBlock with all properties
- Format relationships as wikilinks
- Write "About" section
- Save to VisioningLab/

Use hooks:
- npx claude-flow@alpha hooks pre-task --description "Format [file]"
- npx claude-flow@alpha hooks post-edit --file "[file]"
```

### OWL Agent Prompt:
```
Create OWL axioms for these 2 files:

7. Physics Engine.md → term-id: 20106
8. Policy Engine.md → term-id: 20107

For each file:
- Read formatted file
- Check classifier analysis
- Write minimal OWL axioms:
  - Declaration(Class(mv:ClassName))
  - SubClassOf(mv:ClassName mv:VirtualEntity)
  - SubClassOf(mv:ClassName mv:Object)
- Add domain/layer axioms
- Validate syntax
- Store in memory

Use OWL Functional Syntax in code fence:
```clojure
owl:functional-syntax:: |
  Declaration(...)
```
```

### Validator Agent Prompt:
```
Validate this file and all prior Batch 1.1 work:

9. WebXR API.md → term-id: 20108

Tasks:
- Complete WebXR API.md migration
- Run extractor on all 9 files
- Check for errors/warnings
- Generate validation report
- Store results in memory: swarm/validator/batch-1.1-report
- Update MIGRATION_CHECKLIST.md

Extractor command:
cd logseq-owl-extractor
cargo run --release -- --input ../VisioningLab --output ../test-batch-1.1.ofn --validate
```

---

## 🎯 Deliverables

1. **9 Migrated Files** in VisioningLab/ (updated in-place)
2. **Memory State** with all coordination data
3. **Validation Report** in docs/orchestration/
4. **Updated MIGRATION_CHECKLIST.md** marking Batch 1.1 complete
5. **Extracted OWL** file: test-batch-1.1.ofn

---

## 📅 Timeline

- **T+0min**: Spawn all 4 agents concurrently
- **T+30min**: Classifier completes 3 files
- **T+60min**: Formatter completes 3 files
- **T+90min**: OWL Agent completes 2 files
- **T+120min**: Validator completes 1 file + validation
- **T+150min**: Coordinator reviews, marks batch complete

**Total Estimated Time**: 2.5 hours

---

## ✅ Completion Criteria

Batch 1.1 is COMPLETE when:
- [ ] All 9 files in standardized format
- [ ] All files extract without errors
- [ ] All term-ids assigned and tracked
- [ ] MIGRATION_CHECKLIST.md updated
- [ ] Validation report shows 100% success
- [ ] Memory state shows all agents complete
- [ ] Test extraction file generated

---

**Plan Version**: 1.0
**Created**: 2025-10-14
**Status**: Ready to Execute
