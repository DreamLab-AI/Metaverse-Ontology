# [Concept Name]

## Core Properties

term-id:: [numeric ID]
preferred-term:: [Official Term]
definition:: [Clear, concise definition of the concept]
maturity:: [draft|mature|deprecated]
source:: [[Source Organization or Standard]]

## OWL Classification

owl:physicality-dimension:: [PhysicalEntity|VirtualEntity|HybridEntity]
owl:role-dimension:: [Agent|Object|Process]

**Note:** A reasoner will automatically infer this is a subclass of `mv:[InferredClass]`.

## Ontological Relationships

has-part:: [[Component 1]], [[Component 2]]
is-part-of:: [[Parent Concept]]
requires:: [[Dependency 1]], [[Dependency 2]]
depends-on:: [[Related Concept]]
enables:: [[Enabled Capability 1]], [[Enabled Capability 2]]
binds-to:: [[Bound Entity]] *(only for HybridEntity)*
belongsToDomain:: [[ETSI Domain]]
implementedInLayer:: [[Architecture Layer]]

## OWL Functional Syntax

owl:functional-syntax:: |
  Declaration(Class(mv:[ConceptName]))

  # Assert classification along the two primary dimensions
  SubClassOf(mv:[ConceptName] mv:[PhysicalityDimension])
  SubClassOf(mv:[ConceptName] mv:[RoleDimension])

  # Add domain-specific constraints (if any)
  # Example cardinality constraint:
  # SubClassOf(mv:[ConceptName]
  #   ObjectExactCardinality(1 mv:propertyName mv:RangeClass)
  # )

  # Link to classification schemas
  SubClassOf(mv:[ConceptName]
    ObjectSomeValuesFrom(mv:belongsToDomain mv:[DomainName])
  )
  SubClassOf(mv:[ConceptName]
    ObjectSomeValuesFrom(mv:implementedInLayer mv:[LayerName])
  )

## Logseq Outline View
collapsed:: true
- ### Core Properties
	- term-id:: [numeric ID]
	- preferred-term:: [Official Term]
	- definition:: [Clear definition]
	- maturity:: [draft|mature|deprecated]
	- source:: [[Source]]
- ### OWL Classification
	- owl:physicality-dimension:: [PhysicalEntity|VirtualEntity|HybridEntity]
	- owl:role-dimension:: [Agent|Object|Process]
	- **Inferred:** `mv:[InferredClass]` (via reasoner)
- ### Ontological Relationships
	- has-part:: [[Component 1]], [[Component 2]]
	- is-part-of:: [[Parent]]
	- requires:: [[Dependency 1]]
	- enables:: [[Capability 1]]
	- belongsToDomain:: [[Domain]]
	- implementedInLayer:: [[Layer]]
- ### Formal OWL Axioms
	- `Declaration(Class(mv:[ConceptName]))`
	- **Classification Axioms**
		- `SubClassOf(mv:[ConceptName] mv:[PhysicalityDimension])`
		- `SubClassOf(mv:[ConceptName] mv:[RoleDimension])`
	- **Domain-Specific Constraints** (if applicable)
		- [Description of constraint]
		- [OWL axiom]
	- **Domain Classification**
		- `SubClassOf(mv:[ConceptName] ObjectSomeValuesFrom(mv:belongsToDomain mv:[Domain]))`
		- `SubClassOf(mv:[ConceptName] ObjectSomeValuesFrom(mv:implementedInLayer mv:[Layer]))`

## Usage Notes

[Optional: Add usage examples, implementation notes, or clarifications]

## Related Concepts

- [[Related Concept 1]]
- [[Related Concept 2]]
- [[Related Concept 3]]

## References

- [Standard or specification reference]
- [Research paper or documentation]
