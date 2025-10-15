# Contributing to Metaverse Ontology

Welcome! We're excited that you're interested in contributing to the Metaverse Ontology project. This guide will help you get started with adding new concepts, improving existing ones, and collaborating with the community.

## Table of Contents

- [Welcome & Project Overview](#welcome--project-overview)
- [Ways to Contribute](#ways-to-contribute)
- [Getting Started](#getting-started)
- [Adding New Concepts](#adding-new-concepts)
- [Submission Process](#submission-process)
- [Code Style & Standards](#code-style--standards)
- [Testing & Validation](#testing--validation)
- [Community Guidelines](#community-guidelines)
- [License](#license)

---

## Welcome & Project Overview

The **Metaverse Ontology** is a formal knowledge representation system for metaverse concepts, built using an innovative hybrid approach that combines:

- **Logseq markdown** for human-readable editing and navigation
- **OWL 2 DL** (Web Ontology Language) for formal reasoning and automatic classification
- **Orthogonal classification** using two independent dimensions (Physicality × Role)
- **ETSI domain alignment** following metaverse standardization efforts

### Our Mission

To create a comprehensive, formally validated ontology that:
- Enables semantic interoperability between metaverse platforms
- Supports automatic reasoning and classification
- Provides clear documentation for developers and researchers
- Aligns with international standards (ETSI, ISO)

### Current Status

- **281+ concepts** migrated and validated
- **Zero OWL validation errors** - full OWL 2 DL compliance
- **Interactive WebVOWL visualization** available
- **Production-ready** with comprehensive documentation

---

## Ways to Contribute

### 1. Adding New Concepts

Help expand the ontology by adding new metaverse concepts with:
- Clear definitions and classifications
- Formal OWL axioms
- ETSI domain assignments
- Real-world examples and use cases

### 2. Improving Existing Concepts

Enhance current concepts by:
- Refining definitions for clarity
- Adding missing relationships
- Improving OWL axioms with better constraints
- Adding use cases and examples
- Updating documentation

### 3. Documentation Improvements

Help others understand the ontology:
- Fix typos and clarify explanations
- Add examples and tutorials
- Improve migration guides
- Create visual diagrams

### 4. Bug Reports

Found an issue? Report:
- Incorrect classifications
- OWL syntax errors
- Missing relationships
- Inconsistent terminology
- Extraction tool bugs

### 5. Feature Requests

Suggest improvements:
- New properties or relationships
- Additional classification dimensions
- Tooling enhancements
- Export format support
- Integration capabilities

---

## Getting Started

### Prerequisites

- **Git** for version control
- **Rust** (latest stable) for the extraction tool
- **Logseq** (optional) for visual editing and navigation
- **Protégé** (optional) for OWL visualization

### Fork and Clone

1. **Fork the repository** on GitHub

2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR-USERNAME/OntologyDesign.git
   cd OntologyDesign
   ```

3. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/ORIGINAL-OWNER/OntologyDesign.git
   ```

### Set Up Development Environment

1. **Build the extractor tool:**
   ```bash
   cd logseq-owl-extractor
   cargo build --release
   cd ..
   ```

2. **Verify the build:**
   ```bash
   ./logseq-owl-extractor/target/release/logseq-owl-extractor --help
   ```

3. **Test extraction:**
   ```bash
   ./logseq-owl-extractor/target/release/logseq-owl-extractor \
     --input . \
     --output test-ontology.ofn \
     --validate
   ```

   You should see: `✅ Ontology validation successful`

### Optional: Set Up Logseq

1. Install [Logseq](https://logseq.com/)
2. Open the `OntologyDesign` directory as a graph
3. Navigate concepts using the outline view
4. Use queries to find concepts with `metaverseOntology:: true`

---

## Adding New Concepts

### Step 1: Use the Template

Copy the standard template:

```bash
cp docs/reference/TEMPLATE.md "YourConcept.md"
```

**File naming:**
- Use the concept name as filename (spaces are OK)
- Examples: `Avatar.md`, `Game Engine.md`, `Digital Twin.md`

### Step 2: Classification Guidelines

Every concept must be classified along **two orthogonal dimensions**:

#### Dimension 1: Physicality

Choose ONE:

| Value | Description | Examples |
|-------|-------------|----------|
| `PhysicalEntity` | Has physical form, exists in real world | VR Headset, Motion Capture Rig, Edge Server, Sensor |
| `VirtualEntity` | Purely digital, no physical form | Software, 3D Model, Avatar, Protocol, Smart Contract |
| `HybridEntity` | Binds physical and virtual | Digital Twin, AR Overlay, IoT Device with Virtual Rep |

#### Dimension 2: Role

Choose ONE:

| Value | Description | Examples |
|-------|-------------|----------|
| `Agent` | Autonomous, makes decisions | Avatar, AI Assistant, Autonomous Agent, Bot |
| `Object` | Passive, acted upon | 3D Model, Hardware, Data, Building, Asset |
| `Process` | Activity or transformation | Rendering, Authentication, Synchronization |

#### Inferred Classes (9 Combinations)

The ontology reasoner will automatically infer the intersection class:

| Physicality | Role | Inferred Class | Examples |
|-------------|------|----------------|----------|
| Physical | Agent | `PhysicalAgent` | Human, Robot |
| Physical | Object | `PhysicalObject` | VR Headset, Server |
| Physical | Process | `PhysicalProcess` | Motion Capture |
| Virtual | Agent | `VirtualAgent` | Avatar, AI Bot |
| Virtual | Object | `VirtualObject` | 3D Model, Texture |
| Virtual | Process | `VirtualProcess` | Rendering, Encryption |
| Hybrid | Agent | `HybridAgent` | Cyborg, AI-Controlled Robot |
| Hybrid | Object | `HybridObject` | Digital Twin, AR Object |
| Hybrid | Process | `HybridProcess` | Real-time Sync |

### Step 3: Assign ETSI Domains

Choose one or more domains from:

- `InfrastructureDomain` - Network, compute, cloud, edge infrastructure
- `InteractionDomain` - UX, avatars, immersion, presence
- `TrustAndGovernanceDomain` - Identity, security, privacy
- `ComputationAndIntelligenceDomain` - AI, analytics, data processing
- `CreativeMediaDomain` - 3D content, rendering, authoring
- `VirtualEconomyDomain` - Tokens, NFTs, markets, transactions
- `VirtualSocietyDomain` - Communities, governance, social

See [ETSIDomainClassification.md](ETSIDomainClassification.md) for complete descriptions.

### Step 4: Write OWL Axioms

Use OWL Functional Syntax to define formal constraints:

```clojure
Declaration(Class(mv:YourConcept))

# Classification along two dimensions
SubClassOf(mv:YourConcept mv:VirtualEntity)
SubClassOf(mv:YourConcept mv:Agent)

# Domain classification
SubClassOf(mv:YourConcept
  ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
)

# Add domain-specific constraints (optional)
# Example: cardinality constraint
SubClassOf(mv:YourConcept
  ObjectExactCardinality(1 mv:represents mv:Agent)
)

# Example: property restrictions
SubClassOf(mv:YourConcept
  ObjectSomeValuesFrom(mv:requires mv:RenderingEngine)
)
```

**Key points:**
- Always declare the class first
- Include both physicality and role subclass axioms
- Add domain/layer classifications
- Use cardinality constraints for precise relationships
- Add property restrictions as needed

See [PropertySchema.md](PropertySchema.md) for all available properties.

### Step 5: Complete the Template

Fill in all required fields:

```markdown
- ### OntologyBlock
  id:: yourconcept-ontology
  collapsed:: true
  - metaverseOntology:: true
  - term-id:: [unique number 20000-99999]
  - preferred-term:: Your Concept Name
  - definition:: [Clear one-sentence definition]
  - maturity:: draft
  - source:: [[Source or Standard]]
  - owl:class:: mv:YourConcept
  - owl:physicality:: VirtualEntity
  - owl:role:: Agent
  - owl:inferred-class:: mv:VirtualAgent
  - owl:functional-syntax:: true
  - belongsToDomain:: [[InteractionDomain]]
  - implementedInLayer:: [[ApplicationLayer]]
  - #### Relationships
    id:: yourconcept-relationships
    - has-part:: [[Component1]], [[Component2]]
    - requires:: [[Dependency1]]
    - enables:: [[Capability1]]
  - #### OWL Axioms
    id:: yourconcept-owl-axioms
    collapsed:: true
    - ```clojure
      [Your OWL axioms here]
      ```
- ## About Your Concept
  id:: yourconcept-about
  - [Human-readable description, examples, use cases]
```

### Step 6: Validation Checklist

Before submitting, verify:

- [ ] Filename matches concept name (spaces OK)
- [ ] `### OntologyBlock` heading with `collapsed:: true`
- [ ] `metaverseOntology:: true` is the first property
- [ ] Unique `term-id` assigned (check existing concepts to avoid duplicates)
- [ ] Clear `definition` provided
- [ ] `maturity` set to `draft` (or `mature` if well-established)
- [ ] `owl:physicality` dimension is correct
- [ ] `owl:role` dimension is correct
- [ ] `owl:inferred-class` matches physicality + role combination
- [ ] At least one ETSI domain assigned via `belongsToDomain`
- [ ] OWL Axioms in code fence with ` ```clojure ` syntax
- [ ] `owl:functional-syntax:: true` flag is present
- [ ] File extracts successfully (run extraction tool)
- [ ] OWL syntax validates without errors
- [ ] Human-readable "About" section is informative

---

## Submission Process

### Step 1: Create a Feature Branch

```bash
# Update your fork
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b add-concept-yourconceptname
```

**Branch naming conventions:**
- `add-concept-NAME` - Adding new concept
- `improve-concept-NAME` - Enhancing existing concept
- `fix-NAME` - Bug fixes
- `docs-TOPIC` - Documentation improvements

### Step 2: Run Validation Tests

Before committing, validate your changes:

```bash
# Extract OWL from all concepts
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output validation-test.ofn \
  --validate

# Check for errors
# Should see: ✅ Ontology validation successful
```

**If validation fails:**
- Review error messages for line numbers
- Check OWL Functional Syntax against the specification
- Verify all referenced classes/properties are defined
- Ensure wikilinks are properly formatted

### Step 3: Commit Message Format

Use clear, descriptive commit messages:

```bash
git add YourConcept.md
git commit -m "Add [ConceptName] as [InferredClass]

- Classified as [Physicality] + [Role]
- Belongs to [Domain]
- Includes [key features/constraints]
- Validated OWL syntax"
```

**Commit message structure:**
- **First line:** Summary in imperative mood (50 chars max)
- **Blank line**
- **Body:** Details about classification, domains, key features (wrap at 72 chars)

**Examples:**

```
Add Avatar as VirtualAgent

- Classified as VirtualEntity + Agent
- Belongs to InteractionDomain
- Represents exactly one user or AI agent
- Requires 3D Rendering Engine
- Validated OWL syntax
```

```
Improve Digital Twin with IoT constraints

- Add cardinality constraint for synchronizesWith
- Include real-time data requirements
- Update use cases with industrial examples
- Validated OWL syntax
```

### Step 4: Create Pull Request

Push your branch:

```bash
git push origin add-concept-yourconceptname
```

Create a pull request with:

**Title:** `Add [ConceptName] as [InferredClass]` or `Improve [ConceptName]`

**Description template:**

```markdown
## Summary
Brief description of the concept and its purpose.

## Classification
- **Physicality:** [PhysicalEntity|VirtualEntity|HybridEntity]
- **Role:** [Agent|Object|Process]
- **Inferred Class:** [e.g., VirtualAgent]
- **ETSI Domain(s):** [Domain names]

## Key Features
- Feature 1
- Feature 2
- Feature 3

## Validation
- [ ] OWL extraction successful
- [ ] OWL validation passed
- [ ] No syntax errors
- [ ] Tested in Protégé (if applicable)

## Related Concepts
List any related concepts or dependencies.

## Additional Notes
Any other relevant information.
```

### Step 5: PR Review Process

**What to expect:**

1. **Automated checks** (if configured):
   - OWL extraction and validation
   - Syntax checking

2. **Maintainer review** will check:
   - Classification correctness
   - OWL axiom validity
   - Definition clarity
   - Relationship appropriateness
   - Documentation completeness

3. **Feedback and iteration:**
   - Address reviewer comments
   - Make requested changes
   - Push updates to your branch

4. **Approval and merge:**
   - Once approved, maintainers will merge
   - Your contribution will be part of the ontology!

**Tips for faster review:**
- Follow the template exactly
- Provide clear justification for classifications
- Include examples and use cases
- Reference standards when applicable
- Respond promptly to feedback

---

## Code Style & Standards

### Markdown Formatting

**File structure:**
```markdown
- ### OntologyBlock
  [properties]
  - #### Relationships
  - #### OWL Axioms
- ## About [Concept]
  - ### Key Characteristics
  - ### Technical Components
  - ### Functional Capabilities
  - ### Use Cases
  - ### Standards & References
  - ### Related Concepts
```

**Indentation:**
- Use tabs for outline levels in Logseq format
- Properties are indented under their heading
- Subsections use `####` heading level

**Collapsed sections:**
- `collapsed:: true` on `### OntologyBlock`
- `collapsed:: true` on `#### OWL Axioms`
- Keeps the file tidy when viewed in Logseq

### OWL Functional Syntax Style

**Declarations first:**
```clojure
Declaration(Class(mv:ConceptName))
Declaration(ObjectProperty(mv:propertyName))
```

**Group related axioms:**
```clojure
# Classification
SubClassOf(mv:Concept mv:ParentClass1)
SubClassOf(mv:Concept mv:ParentClass2)

# Constraints
SubClassOf(mv:Concept
  ObjectExactCardinality(1 mv:property mv:Range)
)

# Domain classification
SubClassOf(mv:Concept
  ObjectSomeValuesFrom(mv:belongsToDomain mv:Domain)
)
```

**Formatting:**
- Use comments (`#`) to organize sections
- Indent nested expressions for readability
- Break long axioms across multiple lines
- Use consistent naming (camelCase for classes, kebab-case in wikilinks)

**Naming conventions:**
- Classes: `mv:ClassName` (PascalCase)
- Properties: `mv:propertyName` (camelCase)
- Wikilinks: `[[Page Name]]` (natural language with spaces)

### Wikilink Conventions

**Basic format:**
```markdown
- requires:: [[3D Rendering Engine]]
- has-part:: [[Visual Mesh]], [[Animation Rig]]
- enables:: [[User Embodiment]]
```

**Rules:**
- Use double square brackets: `[[Concept Name]]`
- Use natural language with spaces
- Match the actual filename (case-sensitive in some systems)
- Commas separate multiple links in a list

**The extractor converts wikilinks to IRIs:**
- `[[Game Engine]]` → `mv:GameEngine`
- `[[Digital Twin]]` → `mv:DigitalTwin`
- `[[3D Rendering]]` → `mv:3DRendering`

See [docs/reference/URIMapping.md](docs/reference/URIMapping.md) for conversion rules.

### Properties and Values

**Boolean properties:**
```markdown
metaverseOntology:: true
collapsed:: true
owl:functional-syntax:: true
```

**String values:**
```markdown
preferred-term:: Avatar
definition:: Digital representation of a person...
maturity:: draft
```

**Numeric values:**
```markdown
term-id:: 20067
```

**Wikilink values:**
```markdown
source:: [[ACM + Web3D HAnim]]
belongsToDomain:: [[InteractionDomain]]
```

**List values:**
```markdown
has-part:: [[Component1]], [[Component2]], [[Component3]]
```

---

## Testing & Validation

### Running the Extractor

**Basic extraction:**
```bash
cd /home/john/githubs/OntologyDesign
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output metaverse-ontology.ofn \
  --validate
```

**Extract specific file:**
```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input YourConcept.md \
  --output test.ofn \
  --validate
```

**Expected output:**
```
📖 Parsing markdown files...
🔗 Converting wikilinks to IRIs...
🏗️  Assembling ontology...
✅ Ontology validation successful
💾 Wrote ontology to: metaverse-ontology.ofn
```

### Validating OWL Syntax

**Method 1: Using the extractor (recommended)**
```bash
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . \
  --output test.ofn \
  --validate
```

The `--validate` flag uses `horned-owl` to check:
- Syntax correctness
- OWL 2 DL profile compliance
- IRI validity
- Axiom well-formedness

**Method 2: Using ROBOT (if installed)**
```bash
robot validate --input metaverse-ontology.ofn
```

**Common validation errors:**

| Error | Cause | Fix |
|-------|-------|-----|
| `Unexpected token` | Syntax error in OWL code | Check parentheses, commas, spacing |
| `Undeclared class` | Referenced class not declared | Add `Declaration(Class(mv:ClassName))` |
| `Unknown property` | Property not defined | Check PropertySchema.md for correct name |
| `Invalid IRI` | Malformed identifier | Use valid characters, check namespace |

### Testing in Protégé

**Open the ontology:**
```bash
# Install Protégé from https://protege.stanford.edu/
protege metaverse-ontology.ofn
```

**Verify classification:**
1. Open the ontology in Protégé
2. Go to **Reasoner** → **Pellet** (or HermiT)
3. Click **Start Reasoner**
4. Go to **Entities** → **Classes**
5. Look for inferred subclass relationships

**Expected results:**
- `Avatar` should appear under `VirtualAgent`
- `DigitalTwin` should appear under `HybridObject`
- All concepts should show under their inferred intersection classes

**Check consistency:**
- Protégé will show errors if there are inconsistencies
- Look for red highlighting or error messages
- Fix any disjointness violations or unsatisfiable classes

### Automated Testing (Future)

We're working on:
- GitHub Actions for automatic validation on PR
- Continuous integration with reasoning tests
- Automated consistency checking
- Regression test suite

---

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment.

**Our Standards:**
- Be respectful and considerate
- Welcome diverse perspectives
- Focus on constructive feedback
- Assume good intentions
- Be patient with newcomers

**Unacceptable Behavior:**
- Harassment or discriminatory comments
- Personal attacks or insults
- Publishing private information
- Trolling or inflammatory remarks

**Enforcement:**
- Violations may result in temporary or permanent ban
- Report issues to project maintainers
- All reports will be reviewed confidentially

### Communication Channels

**GitHub Issues:**
- Bug reports
- Feature requests
- Technical discussions
- Questions about specific concepts

**Pull Requests:**
- Code/content contributions
- Discussion of implementation details
- Review and feedback

**GitHub Discussions** (if enabled):
- General questions
- Best practices
- Community announcements

### Getting Help

**Questions about the ontology:**
- Check [docs/guides/QUICKSTART.md](docs/guides/QUICKSTART.md)
- Review [docs/reference/TEMPLATE.md](docs/reference/TEMPLATE.md)
- Look at exemplar files: [Avatar.md](Avatar.md), [DigitalTwin.md](DigitalTwin.md)
- Read [docs/reference/FORMAT_STANDARDIZED.md](docs/reference/FORMAT_STANDARDIZED.md)

**Technical issues:**
- Check [logseq-owl-extractor/README.md](logseq-owl-extractor/README.md)
- Review error messages carefully
- Search existing GitHub issues
- Create new issue with detailed description

**Classification questions:**
- Review [docs/guides/MIGRATION_GUIDE.md](docs/guides/MIGRATION_GUIDE.md)
- Study the orthogonal classification system in the README
- Look at similar concepts for guidance
- Ask in GitHub issues with specific examples

**Standards and references:**
- See [ETSIDomainClassification.md](ETSIDomainClassification.md)
- Check OWL 2 specification: https://www.w3.org/TR/owl2-syntax/
- Review ETSI metaverse standards

### Recognition

**Contributors will be:**
- Listed in project acknowledgments
- Credited in commit history
- Recognized in release notes (for significant contributions)

**Significant contributions include:**
- Adding 5+ validated concepts
- Major documentation improvements
- Tool enhancements
- Bug fixes

---

## License

### Contribution Licensing

By contributing to this project, you agree that your contributions will be licensed under the same license as the project.

**Current License:** MIT or Apache 2.0 (to be confirmed by project maintainers)

**What this means:**
- Your contributions become part of the project
- Contributions are available under open source license
- You retain copyright to your contributions
- You grant rights for project to use your contributions

**Developer Certificate of Origin:**

By making a contribution, you certify that:
- You created the contribution or have rights to submit it
- You understand it will be under the project license
- You have authority to make the contribution
- If someone else contributed, you have noted this in the commit

---

## Quick Reference

### Essential Files

| File | Purpose |
|------|---------|
| [docs/reference/TEMPLATE.md](docs/reference/TEMPLATE.md) | Concept template |
| [Avatar.md](Avatar.md) | VirtualAgent example |
| [DigitalTwin.md](DigitalTwin.md) | HybridObject example |
| [PropertySchema.md](PropertySchema.md) | All OWL properties |
| [ETSIDomainClassification.md](ETSIDomainClassification.md) | Domain taxonomy |
| [docs/guides/MIGRATION_GUIDE.md](docs/guides/MIGRATION_GUIDE.md) | Migration guide |

### Quick Commands

```bash
# Build extractor
cd logseq-owl-extractor && cargo build --release && cd ..

# Extract and validate
./logseq-owl-extractor/target/release/logseq-owl-extractor \
  --input . --output ontology.ofn --validate

# Create branch
git checkout -b add-concept-yourname

# Commit changes
git add YourConcept.md
git commit -m "Add YourConcept as InferredClass"

# Push and create PR
git push origin add-concept-yourname
```

### Classification Quick Reference

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

---

## Thank You!

Thank you for contributing to the Metaverse Ontology project! Your contributions help build a comprehensive, formally validated knowledge base for the metaverse ecosystem.

Every contribution—whether it's a new concept, an improvement, or a bug report—makes this ontology more useful for the community.

**Questions?** Open an issue or reach out to the maintainers.

**Happy contributing!** 🚀
