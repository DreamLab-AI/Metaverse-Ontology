# Metaverse Ontology Schema

## Purpose
This page defines the property schema used across the metaverse glossary to enable consistent ontology representation.

## Core Relationship Properties

### Taxonomic Relationships
- `is-a::` - Direct parent class (e.g., Avatar is-a Digital Representation)
- `subclass-of::` - Broader classification
- `instance-of::` - Specific instantiation

### Compositional Relationships  
- `has-part::` - Component relationships (e.g., Virtual World has-part Avatar)
- `consists-of::` - Aggregation of elements
- `contains::` - Containment relationships

### Dependency Relationships
- `requires::` - Prerequisites (e.g., VR Experience requires VR Headset)
- `depends-on::` - Dependencies
- `uses::` - Utilization relationships

### Capability Relationships
- `enables::` - Capabilities provided (e.g., Blockchain enables Decentralization)
- `supports::` - Support relationships
- `provides::` - Provisions

### Association Relationships
- `related-to::` - General semantic association
- `used-in::` - Application contexts
- `implements::` - Realization of specifications

## Metadata Properties

### Identification
- `term-id::` - Unique numeric identifier from source glossary
- `preferred-term::` - Canonical term name
- `abbreviation::` - Standard abbreviation if applicable

### Documentation
- `definition::` - Formal standards-style definition
- `notes::` - Explanatory context and usage notes
- `example::` - Usage examples

### Classification
- `domain::` - ETSI functional domain classification
- `layer::` - EWG architectural layer
- `aspect::` - MSF aspect or vertical

### Provenance
- `source::` - Primary authoritative source
- `primary-source::` - Main reference document
- `additional-sources::` - Other defining sources
- `also-defined-in::` - Cross-references to other standards

### Status & Maturity
- `maturity::` - Term maturity level (draft / mature / deprecated)
- `version::` - Specification version
- `status::` - Standardization status
- `ontology-status::` - Internal tracking (needs-relationships / complete)

### Terminology
- `synonyms::` - Equivalent alternative terms
- `variant-terms::` - Related term variants
- `see-also::` - Recommended cross-references

## Usage Guidelines

### Creating New Term Pages
1. Use the template structure
2. Fill in all applicable core properties
3. Add at least one ontological relationship
4. Classify by domain and layer
5. Update ontology-status to 'complete' when relationships are defined

### Relationship Cardinality
- Single value: `property:: [[Value]]`
- Multiple values: `property:: [[Value1]], [[Value2]], [[Value3]]`
- List format:
```
property::
  - [[Value1]]
  - [[Value2]]
  - [[Value3]]
```

### Naming Conventions
- Use singular forms for concepts: [[Avatar]] not [[Avatars]]
- Title case for proper concepts
- Lowercase for general attributes
- Create alias pages for common variants

## Validation Queries

### Find Incomplete Terms
```clojure
{{query (property ontology-status needs-relationships)}}
```

### Find Terms Without Domain
```clojure
#+BEGIN_QUERY
{:query [:find ?term
         :where
         [?p :block/name ?term]
         [?p :block/properties ?props]
         [(missing? $ ?p :block/properties :domain)]]
}
#+END_QUERY
```

### Find Orphaned Terms (No Relationships)
```clojure
#+BEGIN_QUERY
{:query [:find ?term
         :where
         [?p :block/name ?term]
         [?p :block/properties ?props]
         [(missing? $ ?p :block/properties :is-a)]
         [(missing? $ ?p :block/properties :has-part)]
         [(missing? $ ?p :block/properties :related-to)]]
}
#+END_QUERY
```

## Maintenance

This schema should be reviewed and updated as:
- New relationship types are identified
- Domain classification evolves
- Standards references are updated
- Community feedback is incorporated

schema-version:: 1.0
last-updated:: [[2025-01-15]]
maintained-by:: Ontology Working Group
