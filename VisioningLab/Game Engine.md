# Game Engine

## Core Properties

term-id:: 20057
preferred-term:: Game Engine
definition:: Software framework providing core functionality for rendering, physics, and interaction in real-time 3D environments.
maturity:: mature
source:: [[Metaverse 101]], [[SIGGRAPH Pipeline WG]], [[OMA3 Media WG]]

## OWL Classification

owl:physicality-dimension:: VirtualEntity
owl:role-dimension:: Object

**Note:** A reasoner will automatically infer this is a subclass of `mv:VirtualObject`.

## Ontological Relationships

has-part:: [[Rendering Pipeline]], [[Physics Engine]], [[Scene Graph]], [[Audio Engine]]
requires:: [[Graphics API]], [[Compute Infrastructure]]
enables:: [[Real-Time Rendering]], [[Interactive Experience]], [[Procedural Content Generation]]
belongsToDomain:: [[InfrastructureDomain]], [[CreativeMediaDomain]]
implementedInLayer:: [[PlatformLayer]]

## OWL Functional Syntax

owl:functional-syntax:: |
  Declaration(Class(mv:GameEngine))

  # Assert classification along the two primary dimensions
  SubClassOf(mv:GameEngine mv:VirtualEntity)
  SubClassOf(mv:GameEngine mv:Object)
  SubClassOf(mv:GameEngine mv:Software)

  # A Game Engine must have at least one rendering capability
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:hasPart mv:RenderingPipeline)
  )

  # A Game Engine must have a physics engine
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:hasPart mv:PhysicsEngine)
  )

  # Link to classification schemas
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:belongsToDomain mv:InfrastructureDomain)
  )
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:belongsToDomain mv:CreativeMediaDomain)
  )
  SubClassOf(mv:GameEngine
    ObjectSomeValuesFrom(mv:implementedInLayer mv:PlatformLayer)
  )

## Logseq Outline View
collapsed:: true
- ### Core Properties
	- term-id:: 20057
	- preferred-term:: Game Engine
	- definition:: Software framework providing core functionality for rendering, physics, and interaction in real-time 3D environments.
	- maturity:: mature
	- source:: [[Metaverse 101]], [[SIGGRAPH Pipeline WG]]
- ### OWL Classification
	- owl:physicality-dimension:: VirtualEntity
	- owl:role-dimension:: Object
	- **Inferred:** `mv:VirtualObject` (via reasoner)
	- **Also subclass of:** `mv:Software`
- ### Ontological Relationships
	- has-part:: [[Rendering Pipeline]], [[Physics Engine]], [[Scene Graph]], [[Audio Engine]]
	- requires:: [[Graphics API]], [[Compute Infrastructure]]
	- enables:: [[Real-Time Rendering]], [[Interactive Experience]]
	- belongsToDomain:: [[InfrastructureDomain]], [[CreativeMediaDomain]]
	- implementedInLayer:: [[PlatformLayer]]
- ### Formal OWL Axioms
	- `Declaration(Class(mv:GameEngine))`
	- **Classification Axioms**
		- `SubClassOf(mv:GameEngine mv:VirtualEntity)`
		- `SubClassOf(mv:GameEngine mv:Object)`
		- `SubClassOf(mv:GameEngine mv:Software)`
	- **Structural Constraints**
		- Must have rendering capability
		- `SubClassOf(mv:GameEngine ObjectSomeValuesFrom(mv:hasPart mv:RenderingPipeline))`
		- Must have physics engine
		- `SubClassOf(mv:GameEngine ObjectSomeValuesFrom(mv:hasPart mv:PhysicsEngine))`
	- **Domain Classification**
		- `SubClassOf(mv:GameEngine ObjectSomeValuesFrom(mv:belongsToDomain mv:InfrastructureDomain))`
		- `SubClassOf(mv:GameEngine ObjectSomeValuesFrom(mv:belongsToDomain mv:CreativeMediaDomain))`

## Usage Notes

Game Engines are foundational software platforms that provide the runtime infrastructure for virtual worlds and interactive experiences. They integrate multiple subsystems (rendering, physics, audio, scripting) into a cohesive framework.

**Common Examples:** Unity, Unreal Engine, Godot, CryEngine

## Related Concepts

- [[Rendering Pipeline]] - Visual output subsystem
- [[Physics Engine]] - Physical simulation subsystem
- [[Scene Graph]] - Spatial data structure
- [[Real-Time Rendering]] - Output process
- [[Virtual World]] - Environment created by engine
- [[Authoring Tool]] - Content creation for engine

## Sources & References

- Metaverse 101 Glossary
- SIGGRAPH Pipeline Working Group
- OMA3 Media Working Group
- Industry standards (Khronos, Unity, Unreal)

## Metadata

imported-from:: [[Metaverse Glossary Excel]]
import-date:: [[2025-01-15]]
ontology-status:: migrated
migration-date:: [[2025-01-14]]
