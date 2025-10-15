# OntologyDefinition

## Design Philosophy
**Metaverse Ontology - Orthogonal Design**
This ontology uses two orthogonal dimensions for classification:

1.  **Physicality Dimension:** `PhysicalEntity`, `VirtualEntity`, or `HybridEntity`
2.  **Role Dimension:** `Agent`, `Object`, or `Process`

This allows for natural multiple inheritance, with a reasoner automatically inferring the nine intersection classes (e.g., `Avatar` ≡ `VirtualEntity` ⊓ `Agent` ≡ `VirtualAgent`).

## Validation Configuration
owl:profile:: OWL2-DL
owl:consistency-required:: true
owl:coherence-required:: true
owl:reasoning-enabled:: true

## OWL Functional Syntax
owl:functional-syntax::
|
  Prefix(mv:=<https://metaverse-ontology.org/>)
  Prefix(owl:=<http://www.w3.org/2002/07/owl#>)
  Prefix(rdf:=<http://www.w3.org/1999/02/22-rdf-syntax-ns#>)
  Prefix(rdfs:=<http://www.w3.org/2000/01/rdf-schema#>)
  Prefix(xsd:=<http://www.w3.org/2001/XMLSchema#>)
  Prefix(dc:=<http://purl.org/dc/elements/1.1/>)
  Prefix(dcterms:=<http://purl.org/dc/terms/>)
  Prefix(etsi:=<https://etsi.org/ontology/>)
  Prefix(iso:=<https://www.iso.org/ontology/>)

  Ontology(<https://metaverse-ontology.org/>
    <https://metaverse-ontology.org/1.0>

    # ========================================
    # METADATA
    # ========================================
    Annotation(rdfs:label "Metaverse Ontology"@en)
    Annotation(dc:description "A formal ontology defining concepts, relationships, and axioms for describing the metaverse. Uses orthogonal classification dimensions for flexible modeling."@en)
    Annotation(dc:creator "Metaverse Ontology Working Group")
    Annotation(dc:date "2025-10-14"^^xsd:date)
    Annotation(dc:license <http://creativecommons.org/licenses/by/4.0/>)
    Annotation(owl:versionInfo "1.0.0")

    # ========================================
    # IMPORTS
    # ========================================
    Import(<http://www.w3.org/2002/07/owl>)

    # ========================================
    # ROOT ENTITY & META-CLASSES
    # ========================================
    Declaration(Class(mv:Entity))
    Annotation(rdfs:comment mv:Entity "The root class for all entities in the ontology.")

    Declaration(Class(mv:AbstractConcept))
    Annotation(rdfs:comment mv:AbstractConcept "Represents abstract concepts or classes (TBox level).")

    Declaration(Class(mv:ConcreteInstance))
    Annotation(rdfs:comment mv:ConcreteInstance "Represents concrete instantiations or individuals (ABox level).")
    DisjointClasses(mv:AbstractConcept mv:ConcreteInstance)

    # ========================================
    # DIMENSION 1: PHYSICALITY
    # ========================================
    Declaration(Class(mv:PhysicalEntity))
    Annotation(rdfs:comment mv:PhysicalEntity "An entity that exists in physical reality with material form.")
    SubClassOf(mv:PhysicalEntity mv:Entity)

    Declaration(Class(mv:VirtualEntity))
    Annotation(rdfs:comment mv:VirtualEntity "An entity that exists only in digital form.")
    SubClassOf(mv:VirtualEntity mv:Entity)

    Declaration(Class(mv:HybridEntity))
    Annotation(rdfs:comment mv:HybridEntity "An entity that necessarily binds both physical and virtual counterparts.")
    SubClassOf(mv:HybridEntity mv:Entity)

    # Axiom: Hybrid entities must bind to at least one physical and one virtual entity.
    SubClassOf(mv:HybridEntity
      ObjectIntersectionOf(
        ObjectSomeValuesFrom(mv:bindsTo mv:PhysicalEntity)
        ObjectSomeValuesFrom(mv:bindsTo mv:VirtualEntity)
      )
    )

    # Physicality classes are mutually disjoint and cover all entities.
    DisjointClasses(mv:PhysicalEntity mv:VirtualEntity mv:HybridEntity)
    EquivalentClasses(mv:Entity
      ObjectUnionOf(mv:PhysicalEntity mv:VirtualEntity mv:HybridEntity)
    )

    # ========================================
    # DIMENSION 2: ROLE
    # ========================================
    Declaration(Class(mv:Agent))
    Annotation(rdfs:comment mv:Agent "An entity capable of autonomous action.")
    SubClassOf(mv:Agent mv:Entity)

    Declaration(Class(mv:Object))
    Annotation(rdfs:comment mv:Object "A passive entity that can be acted upon.")
    SubClassOf(mv:Object mv:Entity)

    Declaration(Class(mv:Process))
    Annotation(rdfs:comment mv:Process "A sequence of activities or transformations.")
    SubClassOf(mv:Process mv:Entity)

    # Role classes are mutually disjoint and cover all entities.
    DisjointClasses(mv:Agent mv:Object mv:Process)
    EquivalentClasses(mv:Entity
      ObjectUnionOf(mv:Agent mv:Object mv:Process)
    )

    # ========================================
    # INTERSECTION CLASSES (for automatic classification)
    # ========================================
    # These 9 classes are defined as intersections of the two orthogonal dimensions.
    # A reasoner will automatically classify entities into these based on their properties.

    # Physicality × Agent
    Declaration(Class(mv:PhysicalAgent))
    EquivalentClasses(mv:PhysicalAgent ObjectIntersectionOf(mv:PhysicalEntity mv:Agent))

    Declaration(Class(mv:VirtualAgent))
    EquivalentClasses(mv:VirtualAgent ObjectIntersectionOf(mv:VirtualEntity mv:Agent))

    Declaration(Class(mv:HybridAgent))
    EquivalentClasses(mv:HybridAgent ObjectIntersectionOf(mv:HybridEntity mv:Agent))

    # Physicality × Object
    Declaration(Class(mv:PhysicalObject))
    EquivalentClasses(mv:PhysicalObject ObjectIntersectionOf(mv:PhysicalEntity mv:Object))

    Declaration(Class(mv:VirtualObject))
    EquivalentClasses(mv:VirtualObject ObjectIntersectionOf(mv:VirtualEntity mv:Object))

    Declaration(Class(mv:HybridObject))
    EquivalentClasses(mv:HybridObject ObjectIntersectionOf(mv:HybridEntity mv:Object))

    # Physicality × Process
    Declaration(Class(mv:PhysicalProcess))
    EquivalentClasses(mv:PhysicalProcess ObjectIntersectionOf(mv:PhysicalEntity mv:Process))

    Declaration(Class(mv:VirtualProcess))
    EquivalentClasses(mv:VirtualProcess ObjectIntersectionOf(mv:VirtualEntity mv:Process))

    Declaration(Class(mv:HybridProcess))
    EquivalentClasses(mv:HybridProcess ObjectIntersectionOf(mv:HybridEntity mv:Process))

    # ========================================
    # TECHNOLOGY STACK ABSTRACTIONS
    # ========================================
    Declaration(Class(mv:Hardware))
    SubClassOf(mv:Hardware mv:PhysicalObject)

    Declaration(Class(mv:Software))
    SubClassOf(mv:Software mv:VirtualObject)

    Declaration(Class(mv:Data))
    SubClassOf(mv:Data mv:VirtualObject)

    # Firmware is defined as Software that runs on Hardware, bridging the two.
    Declaration(Class(mv:Firmware))
    EquivalentClasses(mv:Firmware
      ObjectIntersectionOf(
        mv:Software
        ObjectSomeValuesFrom(mv:runsOn mv:Hardware)
      )
    )
    # Note: Hardware and Software are kept disjoint. Firmware is a type of Software with a specific relation to Hardware.
    DisjointClasses(mv:Hardware mv:Software)
  )