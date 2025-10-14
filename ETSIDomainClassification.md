# ETSI Domain Classification Schema

## Design

This file defines the ETSI functional domains as a formal class hierarchy. Concepts can be linked to these domains via the `belongsToDomain` object property.

## OWL Functional Syntax

owl:functional-syntax:: |

# Root Domain Class

  Declaration(Class(mv:ETSIDomain))
  SubClassOf(mv:ETSIDomain mv:AbstractConcept)

# Linking Property

  Declaration(ObjectProperty(mv:belongsToDomain))
  ObjectPropertyDomain(mv:belongsToDomain mv:Entity)
  ObjectPropertyRange(mv:belongsToDomain mv:ETSIDomain)

# Specific Domain Classes

  Declaration(Class(mv:InfrastructureDomain))
  SubClassOf(mv:InfrastructureDomain mv:ETSIDomain)

  Declaration(Class(mv:InteractionDomain))
  SubClassOf(mv:InteractionDomain mv:ETSIDomain)

  Declaration(Class(mv:TrustAndGovernanceDomain))
  SubClassOf(mv:TrustAndGovernanceDomain mv:ETSIDomain)

  Declaration(Class(mv:ComputationAndIntelligenceDomain))
  SubClassOf(mv:ComputationAndIntelligenceDomain mv:ETSIDomain)
