# Logseq Ontology Edge Labeling Guide

## Overview

Logseq doesn't natively support labeled edges (predicates) in the RDF triple sense (subject-predicate-object), but the community has developed workarounds using **properties** to create semantic relationships that can be queried with Datalog.

## The Problem

Traditional knowledge graphs use RDF triples where relationships have explicit labels - for example, "Tom is-a captain" has three components: Tom (subject), is-a (predicate/edge label), and captain (object). Logseq's default linking creates unlabeled edges that don't distinguish between relationship types.

In the graph view, you can see connections between pages but cannot:
- Distinguish relationship types (e.g., "likes" vs "hates")
- Filter edges by relationship type
- Query by specific predicates

## The Solution: Properties as Edge Labels

The recommended approach is to use **page properties** and **block properties** to store relationship metadata in a key-value format that represents predicates.

### Basic Syntax

Properties use the format `key:: value` and can be added to:
- **Page properties**: First block of a page
- **Block properties**: Any individual block

## Implementation Patterns

### Pattern 1: Page-Level Relationships

Create relationships at the page level to define how entities relate to each other.

**Example: Person relationships**
```markdown
# Page: [[Alice]]

likes:: [[Apple]], [[Chocolate]]
dislikes:: [[Broccoli]]
knows:: [[Bob]], [[Charlie]]
profession:: Software Engineer
```

**Example: Concept relationships**
```markdown
# Page: [[Metaverse]]

is-a:: [[Virtual Environment]]
has-part:: [[Avatar]], [[Virtual World]], [[Digital Assets]]
requires:: [[VR Headset]], [[Internet Connection]]
related-to:: [[Extended Reality]], [[Web3]]
source:: [[ETSI ARF 010]]
```

### Pattern 2: Block-Level Relationships

Use block properties for more granular relationships or when documenting specific instances.

```markdown
- [[Tom]]
  - role:: captain
    start-date:: 2020-01-15
    organization:: [[Starship Enterprise]]
  
- [[Alice]] #person
  relationship-to:: [[Bob]]
  relationship-type:: colleague
  since:: 2019
```

### Pattern 3: Ontology Hierarchies

Build taxonomies using properties to represent class hierarchies.

```markdown
# Page: [[Digital Twin]]

is-a:: [[Virtual Representation]]
subclass-of:: [[Cyber-Physical System]]
has-property:: [[Real-time Synchronization]], [[Bidirectional Data Flow]]
used-in:: [[Manufacturing]], [[Smart Cities]], [[Healthcare]]
term-id:: 20213
domain:: [[ETSI Domain/Computation and Intelligence]]
maturity:: mature
```

### Pattern 4: Multi-valued Properties

Properties can have multiple values for expressing complex relationships.

```markdown
# Page: [[Virtual World]]

synonyms:: [[Metaverse]], [[Virtual Environment]], [[Digital Space]]
related-concepts:: [[Avatar]], [[Immersion]], [[Telepresence]]
standards:: [[IEEE P2048]], [[ISO 23247]], [[ETSI ARF 010]]
domains:: [[Interaction]], [[Trust and Governance]]
```

## Querying Relationships with Datalog

Properties enable powerful querying using Logseq's Datalog-based advanced query system.

### Basic Property Query

Find all pages with a specific property value:

```clojure
#+BEGIN_QUERY
{:title "All Virtual Environments"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :is-a) ?value]
         [(contains? ?value "Virtual Environment")]]
}
#+END_QUERY
```

### Query by Relationship Type

Find all entities that "like" something:

```clojure
#+BEGIN_QUERY
{:title "Who likes what?"
 :query [:find ?entity ?liked
         :where
         [?e :block/properties ?props]
         [?e :block/name ?entity]
         [(get ?props :likes) ?liked]]
}
#+END_QUERY
```

### Filter by Multiple Properties

Combine property filters for complex queries:

```clojure
#+BEGIN_QUERY
{:title "Mature terms in ETSI Domain"
 :query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :maturity) ?maturity]
         [(= ?maturity "mature")]
         [(get ?props :domain) ?domain]
         [(clojure.string/includes? ?domain "ETSI")]
}
#+END_QUERY
```

### Traverse Relationships

Follow relationship chains (e.g., find all parts of parts):

```clojure
#+BEGIN_QUERY
{:title "All components of Metaverse (recursive)"
 :query [:find (pull ?component [*])
         :where
         [?p :block/name "metaverse"]
         [?p :block/properties ?props]
         [(get ?props :has-part) ?parts]
         [?component :block/name ?parts]]
}
#+END_QUERY
```

### Table View with Properties

Display query results as a table with property columns:

```clojure
#+BEGIN_QUERY
{:title "Glossary Terms Overview"
 :query [:find ?term ?definition ?domain ?maturity
         :where
         [?b :block/properties ?props]
         [?b :block/name ?term]
         [(get ?props :definition) ?definition]
         [(get ?props :domain) ?domain]
         [(get ?props :maturity) ?maturity]]
 :view (fn [results]
         [:table
          [:thead [:tr [:th "Term"] [:th "Definition"] [:th "Domain"] [:th "Maturity"]]]
          [:tbody (for [[term def domain mat] results]
                   [:tr [:td term] [:td def] [:td domain] [:td mat]])]])
}
#+END_QUERY
```

## Best Practices for Ontology Work

### 1. Establish a Property Schema

Define a consistent set of property keys across your graph to maintain semantic consistency.

Create a schema page:
```markdown
# Ontology Schema

## Relationship Properties
- `is-a::` - taxonomic parent class
- `has-part::` - mereological composition
- `related-to::` - general association
- `subclass-of::` - direct subclass relationship
- `instance-of::` - instantiation relationship

## Metadata Properties
- `domain::` - ETSI domain classification
- `maturity::` - term maturity (draft/mature/deprecated)
- `source::` - authoritative source
- `term-id::` - unique identifier
- `synonyms::` - alternative terms
```

### 2. Disable Property Pages (Optional)

By default, Logseq creates pages for all property keys. You can disable this behavior in your config.edn:

```clojure
;; In config.edn
:property-pages/enabled? false
```

This prevents cluttering your graph with property name pages.

### 3. Use Templates for Consistency

Create templates for common entity types:

```markdown
# Template: Concept Definition

term-id:: 
preferred-term:: 
definition:: 
is-a:: 
has-part:: 
domain:: 
maturity:: draft
source:: 
synonyms:: 
notes::
```

### 4. Combine with Tags for Flexibility

Use both properties and tags for maximum compatibility:

```markdown
- [[Digital Twin]] #concept #technology
  is-a:: [[Virtual Representation]]
  domain:: [[Computation and Intelligence]]
  tags:: #concept, #technology
```

Tags provide visual markers and work in simpler queries, while properties enable structured querying.

## Limitations and Workarounds

### Limitation 1: No Edge Visualization

Block properties don't show up as labeled edges in the graph view - only page-to-page links are visualized.

**Workaround**: 
- Use explicit page links alongside properties when visualization matters
- Export to tools like Gephi for advanced graph visualization
- Consider the graph view as supplementary to query-based exploration

### Limitation 2: Property Inheritance

Block properties are not inherited by child blocks. Links within properties are inherited, but the key-value pairs themselves are not.

**Workaround**:
- Use page properties for entity-level relationships
- Explicitly set properties on child blocks when needed
- Use links in property values to enable inheritance of those specific references

### Limitation 3: No Native RDF Export

Logseq uses DataScript (a Datalog quad store), not a native RDF triple store, though there are ongoing community efforts to enable RDF/JSON-LD export.

**Workaround**:
- Export to EDN format and convert to JSON-LD using external tools
- Use custom scripts to transform property-based relationships into RDF triples
- Wait for community plugins that provide RDF export functionality

## Advanced: Simulating Typed Edges in Queries

You can create rule-based queries that simulate traversing typed edges:

```clojure
#+BEGIN_QUERY
{:title "Find all components (has-part relationships)"
 :query [:find ?parent ?child
         :in $ %
         :where
         (has-part ?parent ?child)]
 :rules [
   [(has-part ?p ?c)
    [?p :block/properties ?props]
    [(get ?props :has-part) ?parts]
    [(contains? ?parts ?c)]]
 ]
}
#+END_QUERY
```

This creates reusable relationship patterns that act like labeled edges.

## Real-World Example: Metaverse Glossary

Here's how you might structure your metaverse glossary with ontology support:

```markdown
# Page: [[Virtual World]]

## Core Properties
term-id:: 20045
preferred-term:: Virtual World
definition:: Computer-generated simulated environment that users can interact with through digital avatars
maturity:: mature
source:: [[ETSI ARF 010]]

## Ontological Relationships
is-a:: [[Digital Environment]]
has-part:: [[Avatar]], [[Virtual Object]], [[Spatial Audio]], [[Physics Engine]]
requires:: [[Rendering Engine]], [[Network Infrastructure]]
enables:: [[Telepresence]], [[Social Interaction]], [[Commerce]]

## Classification
domain:: [[ETSI Domain/Interaction]]
layer:: [[User Experience Layer]]
aspect:: [[MSF/Social]]

## Related Concepts
related-to:: [[Metaverse]], [[Extended Reality]], [[Digital Twin]]
synonyms:: [[Virtual Environment]], [[Synthetic World]]
see-also:: [[Persistent World]], [[Shared Virtual Space]]

## Standards
standards-reference:: [[IEEE P2048.1]], [[ISO 23257]]
schema-org-type:: VirtualLocation
```

## Query Examples for Metaverse Glossary

### Find All Terms in a Domain
```clojure
{{query (property domain "ETSI Domain/Interaction")}}
```

### Find All Mature Terms with Sources
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
         :where
         [?p :block/properties ?props]
         [(get ?props :maturity) ?m]
         [(= ?m "mature")]
         [(get ?props :source)]]
}
#+END_QUERY
```

### Build a Hierarchical View
```clojure
#+BEGIN_QUERY
{:title "Concept Hierarchy"
 :query [:find ?term ?parent
         :where
         [?p :block/properties ?props]
         [?p :block/name ?term]
         [(get ?props :is-a) ?parent]]
 :view (fn [results]
         [:ul
          (for [[term parent] (sort-by first results)]
            [:li term " → " parent])])
}
#+END_QUERY
```

## Conclusion

While Logseq doesn't have native RDF-style edge labeling, the property system combined with Datalog queries provides a powerful way to represent and query ontological relationships in markdown format. This approach:

✅ Maintains plain-text markdown compatibility  
✅ Enables sophisticated semantic queries  
✅ Supports ontology development workflows  
✅ Scales to large knowledge bases  
✅ Integrates with existing Logseq features  

The trade-off is that relationships aren't visually represented as labeled edges in the graph view, but the querying capabilities more than compensate for this limitation in most ontology work scenarios.

## Further Resources

- **Logseq Queries Tutorial**: https://discuss.logseq.com/t/lesson-5-how-to-power-your-workflows-using-properties-and-dynamic-variables/10173
- **Learn Datalog Today**: http://www.learndatalogtoday.org/
- **DataScript Documentation**: https://github.com/tonsky/datascript
- **Community Discussion on RDF/Semantic Web**: https://discuss.logseq.com/t/rdf-json-ld-tripple-store-schema-org-etc/13051
- **Edge Labeling Discussion**: https://discuss.logseq.com/t/edge-labeling-edge-predicate-knowledge/14108
