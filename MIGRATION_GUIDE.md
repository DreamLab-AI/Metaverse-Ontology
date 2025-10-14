# Migration Guide: VisioningLab Files to Standard Format

This guide explains how to migrate the 260+ files in the VisioningLab directory to the standardized format that supports both programmatic processing and Logseq navigation.

## Format Requirements

Each concept file must have:

1. **Standard markdown sections** (lines 1-46 in Avatar.md)
   - Machine-readable by the extractor tool
   - Properties in `key:: value` format
   - OWL Functional Syntax block with `owl:functional-syntax:: |`

2. **Logseq Outline View** (lines 48-76 in Avatar.md)
   - Collapsed by default for clean Logseq navigation
   - Nested bullet structure
   - Human-readable format with inline code formatting

## Structure Overview

```markdown
# Concept Name                          # Page title

## Core Properties                      # Extractable properties
term-id:: 12345
definition:: ...
[other properties]

## OWL Classification                   # Classification dimensions
owl:physicality-dimension:: ...
owl:role-dimension:: ...

## Ontological Relationships            # Wikilinks to related concepts
has-part:: [[A]], [[B]]
requires:: [[C]]

## OWL Functional Syntax                # Machine-readable OWL
owl:functional-syntax:: |
  Declaration(...)
  SubClassOf(...)

## Logseq Outline View                  # Human-readable outline
collapsed:: true
- ### Core Properties
  - term-id:: 12345
  - [nested properties]
- ### OWL Classification
  - [classification info]
- ### Ontological Relationships
  - [relationships]
- ### Formal OWL Axioms
  - [axioms in readable format]
```

## Migration Steps

### Step 1: Identify Current Format

Examine each file to determine its current structure:

```bash
cd VisioningLab
head -20 "Avatar.md"
```

Common formats you'll encounter:
- Pure Logseq outline (all bullets)
- Pure markdown (no outline section)
- Mixed format (like old Avatar.md)

### Step 2: Extract Core Information

For each file, identify:

1. **Concept name** (from filename or heading)
2. **Properties** (term-id, definition, maturity, source)
3. **Classification** (physicality + role dimensions)
4. **Relationships** (has-part, requires, enables, etc.)
5. **Existing OWL axioms** (if any)

### Step 3: Determine OWL Classification

For each concept, decide:

**Physicality Dimension:**
- `PhysicalEntity` - Has physical form (hardware, sensors, displays)
- `VirtualEntity` - Purely digital (software, data, virtual objects)
- `HybridEntity` - Binds physical and virtual (Digital Twin, AR overlay)

**Role Dimension:**
- `Agent` - Autonomous actors (AI agents, avatars, autonomous systems)
- `Object` - Passive entities (assets, data, hardware)
- `Process` - Activities/transformations (rendering, authentication, synchronization)

**Resulting Classes:**
| Physicality | Role | Result |
|-------------|------|--------|
| Physical | Agent | PhysicalAgent |
| Physical | Object | PhysicalObject |
| Physical | Process | PhysicalProcess |
| Virtual | Agent | VirtualAgent |
| Virtual | Object | VirtualObject |
| Virtual | Process | VirtualProcess |
| Hybrid | Agent | HybridAgent |
| Hybrid | Object | HybridObject |
| Hybrid | Process | HybridProcess |

### Step 4: Map to ETSI Domains

Classify each concept into one or more ETSI domains:

- `InfrastructureDomain` - Network, compute, edge, cloud
- `InteractionDomain` - UI/UX, avatars, immersion, presence
- `TrustAndGovernanceDomain` - Identity, security, privacy, compliance
- `ComputationAndIntelligenceDomain` - AI, data processing, analytics
- `CreativeMediaDomain` - Content creation, 3D assets, rendering
- `VirtualEconomyDomain` - Tokens, NFTs, transactions, markets
- `VirtualSocietyDomain` - Communities, governance, social structures

### Step 5: Apply Template

Use [TEMPLATE.md](TEMPLATE.md) as a guide. Replace placeholders with actual values.

### Step 6: Generate OWL Axioms

Write OWL Functional Syntax axioms:

```
Declaration(Class(mv:ConceptName))
SubClassOf(mv:ConceptName mv:PhysicalityDimension)
SubClassOf(mv:ConceptName mv:RoleDimension)
```

Add domain-specific constraints as needed.

### Step 7: Create Logseq Outline

Create the collapsed outline section for Logseq navigation.

## Batch Processing Approach

Given 260+ files, process in batches:

### Batch 1: Simple Virtual Objects (30-40 files)
- 3D models, textures, data formats
- Examples: glTF, Scene Graph, Metadata Standard
- Classification: `VirtualEntity` + `Object`

### Batch 2: Virtual Processes (30-40 files)
- Algorithms, rendering, protocols
- Examples: Rendering Pipeline, State Synchronization, Procedural Generation
- Classification: `VirtualEntity` + `Process`

### Batch 3: Virtual Agents (10-20 files)
- AI agents, autonomous systems, bots
- Examples: Autonomous Agent, Intelligent Virtual Entity
- Classification: `VirtualEntity` + `Agent`

### Batch 4: Physical Objects (20-30 files)
- Hardware, sensors, devices
- Examples: VR Headset, Edge Computing Node, Motion Capture Rig
- Classification: `PhysicalEntity` + `Object`

### Batch 5: Hybrid Objects (10-20 files)
- Digital Twins, AR objects
- Examples: Digital Twin, Construction Digital Twin
- Classification: `HybridEntity` + `Object`

### Batch 6: Infrastructure & Networking (30-40 files)
- Networking, compute, cloud services
- Mixed classifications, mostly Process or Object

### Batch 7: Governance & Trust (30-40 files)
- Identity, security, compliance
- Mixed classifications

### Batch 8: Economy & Society (30-40 files)
- Tokens, marketplaces, governance
- Mixed classifications

### Batch 9: Domain Classes (20-30 files)
- ETSI_Domain_* files
- Special handling: These are AbstractConcept, not Entity

### Batch 10: Applications (10-20 files)
- Health, Education, Tourism metaverses
- Mixed classifications

## Classification Decision Tree

Use this flowchart for each concept:

```
1. Does it have physical form?
   ├─ YES → PhysicalEntity
   ├─ NO → VirtualEntity
   └─ BOTH (binds physical to virtual) → HybridEntity

2. What is its primary role?
   ├─ Makes autonomous decisions? → Agent
   ├─ Passive, can be acted upon? → Object
   └─ Represents activity/transformation? → Process

3. Which ETSI domain?
   - Check domain definitions in ETSIDomainClassification.md
   - A concept can belong to multiple domains
```

## Example Migrations

### Example 1: Game Engine

**Before:** (Simple markdown)
```markdown
# Game Engine
- Software framework for creating games and interactive experiences
```

**After:** (Standard format)
```markdown
# Game Engine

## Core Properties
term-id:: 30100
preferred-term:: Game Engine
definition:: Software framework providing rendering, physics, and scripting capabilities for creating interactive virtual experiences.
maturity:: mature
source:: [[Industry Standard]]

## OWL Classification
owl:physicality-dimension:: VirtualEntity
owl:role-dimension:: Object

## Ontological Relationships
has-part:: [[Physics Engine]], [[Rendering Pipeline]], [[Scene Graph]]
enables:: [[Procedural Content Generation]], [[Real-Time Rendering]]
belongsToDomain:: [[InfrastructureDomain]]
implementedInLayer:: [[PlatformLayer]]

## OWL Functional Syntax
owl:functional-syntax:: |
  Declaration(Class(mv:GameEngine))
  SubClassOf(mv:GameEngine mv:VirtualEntity)
  SubClassOf(mv:GameEngine mv:Object)
  SubClassOf(mv:GameEngine mv:Software)
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:belongsToDomain mv:InfrastructureDomain)
  )

## Logseq Outline View
collapsed:: true
- ### Core Properties
  - term-id:: 30100
  - preferred-term:: Game Engine
  - definition:: Software framework for virtual experiences
  - maturity:: mature
- ### OWL Classification
  - owl:physicality-dimension:: VirtualEntity
  - owl:role-dimension:: Object
  - **Inferred:** `mv:VirtualObject` (via reasoner)
- ### Ontological Relationships
  - has-part:: [[Physics Engine]], [[Rendering Pipeline]]
  - enables:: [[Real-Time Rendering]]
  - belongsToDomain:: [[InfrastructureDomain]]
```

### Example 2: Digital Twin

Already complete in [DigitalTwin.md](DigitalTwin.md) - use as reference for HybridEntity concepts.

### Example 3: Smart Contract

**Classification:**
- Physicality: `VirtualEntity` (code running on blockchain)
- Role: `Process` (executes transactions)
- Domain: `VirtualEconomyDomain` + `TrustAndGovernanceDomain`

## Automation Strategy

### Semi-Automated Approach

1. **Create a Python script** to:
   - Parse existing files
   - Extract properties and relationships
   - Generate template-based output
   - Flag items needing human review

2. **Human review** for:
   - OWL classification (physicality + role)
   - Domain assignment
   - Complex axioms
   - Definition refinement

3. **Validation script** to:
   - Check all required sections present
   - Validate property syntax
   - Check wikilink consistency
   - Verify OWL syntax

### Python Helper Script (Skeleton)

```python
# migrate_concept.py
import re
import sys

def extract_title(content):
    match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    return match.group(1) if match else "Unknown"

def extract_properties(content):
    # Extract term-id, definition, etc.
    pass

def classify_concept(title, content):
    # Heuristics for classification
    # Returns (physicality, role, domains)
    pass

def generate_standard_format(concept_data):
    # Use TEMPLATE.md to generate output
    pass

if __name__ == "__main__":
    input_file = sys.argv[1]
    with open(input_file) as f:
        content = f.read()

    # Extract and classify
    concept_data = parse_concept(content)

    # Generate output
    output = generate_standard_format(concept_data)

    print(output)
```

## Quality Checklist

For each migrated file, verify:

- [ ] Title matches concept name
- [ ] term-id is unique and numeric
- [ ] definition is clear and concise
- [ ] maturity is set (draft/mature/deprecated)
- [ ] OWL classification dimensions are correct
- [ ] At least one ETSI domain assigned
- [ ] Wikilinks use correct `[[Page Name]]` syntax
- [ ] OWL Functional Syntax block is valid
- [ ] Logseq outline section has `collapsed:: true`
- [ ] All sections present and properly formatted
- [ ] File can be extracted by logseq-owl-extractor

## Testing

After migration of each batch:

```bash
# Run extractor
cd logseq-owl-extractor
cargo run --release -- --input ../VisioningLab --output test-batch.ofn --validate

# Check for errors
grep -i error test-batch.log

# Load in Protégé for visual inspection
protege test-batch.ofn
```

## Common Issues and Solutions

### Issue 1: Duplicate Content
**Problem:** Old files have content in both standard and outline formats
**Solution:** Keep standard format (top), create NEW outline section (bottom)

### Issue 2: Inconsistent Property Names
**Problem:** `has-part` vs `hasPart` vs `has_part`
**Solution:** Always use kebab-case in properties: `has-part`

### Issue 3: Missing Classifications
**Problem:** File doesn't specify physicality/role dimensions
**Solution:** Analyze concept meaning, assign appropriate dimensions

### Issue 4: Broken Wikilinks
**Problem:** Links to non-existent pages
**Solution:** Create placeholder pages or fix link names

### Issue 5: Invalid OWL Syntax
**Problem:** Malformed axioms in OWL block
**Solution:** Use Protégé or horned-owl to validate syntax

## Progress Tracking

Create a spreadsheet to track:

| Filename | Status | Physicality | Role | Domain | Reviewer | Date |
|----------|--------|-------------|------|--------|----------|------|
| Avatar.md | ✅ Done | Virtual | Agent | Interaction | John | 2025-01-14 |
| Digital Twin.md | ✅ Done | Hybrid | Object | Infrastructure | John | 2025-01-14 |
| Game Engine.md | 🔄 In Progress | Virtual | Object | Infrastructure | - | - |
| ... | | | | | | |

## Timeline Estimate

- **Per file:** 5-15 minutes (depending on complexity)
- **Simple concepts:** 5 min
- **Medium concepts:** 10 min
- **Complex concepts:** 15-20 min
- **Total for 260 files:** 40-65 hours of work

**Recommendation:** Process 10-20 files per session, with breaks for validation.

## Next Steps

1. Read this guide thoroughly
2. Review [TEMPLATE.md](TEMPLATE.md)
3. Study [Avatar.md](Avatar.md) as exemplar
4. Start with Batch 1 (Simple Virtual Objects)
5. Run extractor and validator after each batch
6. Iterate and refine

---

**Need help?** Refer to:
- [TEMPLATE.md](TEMPLATE.md) - Standard format template
- [Avatar.md](Avatar.md) - Perfect example (VirtualAgent)
- [DigitalTwin.md](DigitalTwin.md) - Perfect example (HybridObject)
- [URIMapping.md](URIMapping.md) - Wikilink conversion rules
