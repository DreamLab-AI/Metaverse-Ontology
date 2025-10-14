# Metaverse Ontology Property Schema

This file provides the formal OWL declarations for all Object, Data, and Annotation properties.

## Object Properties

### Mereological (Part-Whole)

#### hasPart

owl:iri:: mv:hasPart
owl:type:: ObjectProperty
owl:characteristics:: Transitive, Irreflexive, Asymmetric
owl:inverse-of:: mv:isPartOf
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:hasPart))
  TransitiveObjectProperty(mv:hasPart)
  IrreflexiveObjectProperty(mv:hasPart)
  AsymmetricObjectProperty(mv:hasPart)

#### isPartOf

owl:iri:: mv:isPartOf
owl:type:: ObjectProperty
owl:characteristics:: Transitive
owl:inverse-of:: mv:hasPart
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:isPartOf))
  TransitiveObjectProperty(mv:isPartOf)
  InverseObjectProperties(mv:hasPart mv:isPartOf)

### Dependency

#### dependsOn

owl:iri:: mv:dependsOn
owl:type:: ObjectProperty
owl:characteristics:: Transitive
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:dependsOn))
  TransitiveObjectProperty(mv:dependsOn)

#### requires

owl:iri:: mv:requires
owl:type:: ObjectProperty
owl:sub-property-of:: mv:dependsOn
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:requires))
  SubObjectPropertyOf(mv:requires mv:dependsOn)

### Capability

#### enables

owl:iri:: mv:enables
owl:type:: ObjectProperty
owl:inverse-of:: mv:enabledBy
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:enables))

#### enabledBy

owl:iri:: mv:enabledBy
owl:type:: ObjectProperty
owl:inverse-of:: mv:enables
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:enabledBy))
  InverseObjectProperties(mv:enables mv:enabledBy)

### Binding (for Hybrid Entities)

#### bindsTo

owl:iri:: mv:bindsTo
owl:type:: ObjectProperty
owl:characteristics:: Symmetric, Irreflexive, Functional
owl:domain:: ObjectUnionOf(mv:PhysicalEntity mv:VirtualEntity mv:HybridEntity)
owl:range:: ObjectUnionOf(mv:PhysicalEntity mv:VirtualEntity)
owl:functional-syntax:: |
  Declaration(ObjectProperty(mv:bindsTo))
  SymmetricObjectProperty(mv:bindsTo)
  IrreflexiveObjectProperty(mv:bindsTo)
  FunctionalObjectProperty(mv:bindsTo)
  ObjectPropertyDomain(mv:bindsTo ObjectUnionOf(mv:PhysicalEntity mv:VirtualEntity mv:HybridEntity))
  ObjectPropertyRange(mv:bindsTo ObjectUnionOf(mv:PhysicalEntity mv:VirtualEntity))

## Data Properties

### Identification

#### termId

owl:iri:: mv:termId
owl:type:: DataProperty
owl:characteristics:: Functional
owl:range:: xsd:integer
owl:functional-syntax:: |
  Declaration(DataProperty(mv:termId))
  FunctionalDataProperty(mv:termId)
  DataPropertyRange(mv:termId xsd:integer)

### Classification

#### maturity

owl:iri:: mv:maturity
owl:type:: DataProperty
owl:range:: mv:MaturityLevel (Enumerated Datatype)
owl:functional-syntax:: |
  Declaration(DataProperty(mv:maturity))
  Declaration(Datatype(mv:MaturityLevel))
  DatatypeDefinition(mv:MaturityLevel DataOneOf("draft"^^xsd:string "mature"^^xsd:string "deprecated"^^xsd:string))
  DataPropertyRange(mv:maturity mv:MaturityLevel)

## Annotation Properties

#### source

owl:iri:: mv:source
owl:type:: AnnotationProperty
owl:functional-syntax:: |
  Declaration(AnnotationProperty(mv:source))

#### synonyms

owl:iri:: mv:synonyms
owl:type:: AnnotationProperty
owl:functional-syntax:: |
  Declaration(AnnotationProperty(mv:synonyms))
