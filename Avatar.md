
# Avatar

## Core Properties

term-id:: 20067
preferred-term:: Avatar
definition:: Digital representation of a person or agent used to interact within a virtual environment.
maturity:: mature
source:: [[ACM + Web3D HAnim]]

## OWL Classification

owl:physicality-dimension:: VirtualEntity
owl:role-dimension:: Agent

# A reasoner will automatically infer this is a subclass of mv:VirtualAgent.

## Ontological Relationships

has-part:: [[Visual Mesh]], [[Animation Rig]]
requires:: [[3D Rendering Engine]]
enables:: [[User Embodiment]], [[Social Presence]]
belongsToDomain:: [[InteractionDomain]]
implementedInLayer:: [[UserExperienceLayer]]

## OWL Functional Syntax

owl:functional-syntax:: |
  Declaration(Class(mv:Avatar))

# Assert classification along the two primary dimensions

  SubClassOf(mv:Avatar mv:VirtualEntity)
  SubClassOf(mv:Avatar mv:Agent)

# Add specific domain knowledge (cardinality constraint)

# An Avatar must represent exactly one Agent (e.g., a user or an AI).

  SubClassOf(mv:Avatar
    ObjectExactCardinality(1 mv:represents mv:Agent)
  )

# Link to classification schemas

  SubClassOf(mv:Avatar
    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
  )
  SubClassOf(mv:Avatar
    ObjectSomeValuesFrom(mv:implementedInLayer mv:UserExperienceLayer)
  )
