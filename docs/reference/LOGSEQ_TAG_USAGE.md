# Logseq Tag Usage: `metaverseOntology:: true`

## Overview

Every ontology concept file must include the `metaverseOntology:: true` tag at the beginning of the OntologyBlock. This tag enables powerful Logseq features for managing and navigating the ontology.

## Tag Location

The tag should be the **first property** in the OntologyBlock:

```markdown
- OntologyBlock
  collapsed:: true
	- metaverseOntology:: true    ← FIRST property
	- term-id:: 20067
	- preferred-term:: Avatar
	- [other properties...]
```

## Benefits

### 1. Filtering and Querying

Query all ontology concepts in Logseq:

```clojure
#+BEGIN_QUERY
{:title "All Metaverse Ontology Concepts"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]]}
#+END_QUERY
```

### 2. Find by Classification

Query concepts by physicality dimension:

```clojure
#+BEGIN_QUERY
{:title "All Virtual Entities"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [(get ?props :owl:physicality) ?phys]
         [(= ?phys "VirtualEntity")]]}
#+END_QUERY
```

### 3. Find by Domain

Query concepts in a specific ETSI domain:

```clojure
#+BEGIN_QUERY
{:title "Infrastructure Domain Concepts"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [?p :block/refs ?r]
         [?r :block/name "infrastructuredomain"]]}
#+END_QUERY
```

### 4. Maturity Tracking

Find draft concepts that need work:

```clojure
#+BEGIN_QUERY
{:title "Draft Ontology Concepts"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [(get ?props :maturity) ?m]
         [(= ?m "draft")]]}
#+END_QUERY
```

### 5. Linked References Graph

The tag creates a central page for all ontology concepts. In Logseq:

1. Navigate to the `metaverseOntology` page (auto-created)
2. View "Linked References" section
3. See all concepts in the ontology at once
4. Visualize relationships in graph view

### 6. Custom Dashboards

Create a custom dashboard page in Logseq:

```markdown
# Metaverse Ontology Dashboard

## Statistics
- Total concepts: {{query (and (property metaverseOntology true))}}
- Virtual entities: {{query (and (property metaverseOntology true) (property owl:physicality VirtualEntity))}}
- Hybrid entities: {{query (and (property metaverseOntology true) (property owl:physicality HybridEntity))}}

## By Maturity
- Draft: {{query (and (property metaverseOntology true) (property maturity draft))}}
- Mature: {{query (and (property metaverseOntology true) (property maturity mature))}}

## By Domain
- Infrastructure: {{query (and (property metaverseOntology true) [[InfrastructureDomain]])}}
- Interaction: {{query (and (property metaverseOntology true) [[InteractionDomain]])}}
```

## Example Queries

### All Agents (Virtual, Physical, Hybrid)

```clojure
#+BEGIN_QUERY
{:title "All Agent Concepts"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [(get ?props :owl:role) ?role]
         [(= ?role "Agent")]]}
#+END_QUERY
```

### Concepts Requiring Specific Component

Find all concepts that require "3D Rendering Engine":

```clojure
#+BEGIN_QUERY
{:title "Concepts Requiring 3D Rendering"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [?p :block/refs ?r]
         [?r :block/name "3d rendering engine"]
         [?p :block/content ?c]
         [(clojure.string/includes? ?c "requires::")]]}
#+END_QUERY
```

### Recently Modified Ontology Concepts

```clojure
#+BEGIN_QUERY
{:title "Recently Updated Ontology Concepts"
 :query [:find (pull ?p [*])
         :in $ ?start ?today
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [?p :block/updated-at ?updated]
         [(>= ?updated ?start)]
         [(<= ?updated ?today)]]
 :inputs [:7d-before :today]}
#+END_QUERY
```

## Integration with Extractor

The `metaverseOntology:: true` property is:

- ✅ **Ignored by the extractor** - Not converted to OWL axioms
- ✅ **Preserved in markdown** - Remains in source files
- ✅ **Used by Logseq** - For queries, filtering, and navigation
- ✅ **Human-readable** - Clear intent when reading files

The extractor focuses only on OWL-specific properties:
- `owl:class`, `owl:physicality`, `owl:role`
- `term-id`, `definition`, `maturity`
- Relationships and OWL axiom blocks

## Migration Checklist

When migrating VisioningLab files, ensure:

- [ ] `metaverseOntology:: true` is first property in OntologyBlock
- [ ] Tag is exactly `metaverseOntology` (case-sensitive in queries)
- [ ] Value is exactly `true` (lowercase boolean)
- [ ] Tag appears before term-id and other properties
- [ ] File tests successfully with extractor (tag doesn't break extraction)

## Graph Visualization

With this tag, you can create powerful graph views in Logseq:

1. **Settings → Features → Enable "Graph View"**
2. Click the graph icon in the sidebar
3. Search for "metaverseOntology"
4. See all ontology concepts and their connections

Filter options:
- Show only nodes with `metaverseOntology` tag
- Color by domain (Infrastructure, Interaction, etc.)
- Size by number of linked references
- Highlight by maturity level

## Advanced Queries

### Concepts by Inferred Class

```clojure
#+BEGIN_QUERY
{:title "Virtual Objects"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [(get ?props :owl:inferred-class) ?inferred]
         [(= ?inferred "mv:VirtualObject")]]}
#+END_QUERY
```

### Cross-Domain Concepts

Find concepts belonging to multiple domains:

```clojure
#+BEGIN_QUERY
{:title "Cross-Domain Concepts"
 :query [:find (pull ?p [:block/name :block/properties])
         :where
         [?p :block/properties ?props]
         [(get ?props :metaverseontology) ?v]
         [(= ?v true)]
         [(get ?props :belongstodomain) ?domains]
         [(clojure.string/includes? ?domains ",")]]}
#+END_QUERY
```

## Best Practices

1. **Always add the tag first** - Makes it easy to spot in files
2. **Use lowercase `true`** - Consistent with Logseq conventions
3. **Don't change the tag name** - Queries depend on exact spelling
4. **Test queries** - Verify tag works after migration
5. **Document custom queries** - Share useful queries with team

## See Also

- [TEMPLATE.md](TEMPLATE.md) - Template with tag included
- [Avatar.md](Avatar.md) - Example with tag
- [DigitalTwin.md](DigitalTwin.md) - Example with tag
- [VisioningLab/Game Engine.md](VisioningLab/Game%20Engine.md) - Example with tag
- [FORMAT_STANDARDIZED.md](FORMAT_STANDARDIZED.md) - Complete format documentation

---

**Tag:** `metaverseOntology:: true`
**Location:** First property in OntologyBlock
**Purpose:** Enable Logseq filtering, querying, and visualization
**Required:** Yes, for all ontology concept files
