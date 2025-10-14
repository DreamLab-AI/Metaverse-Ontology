# Ontology Validation Test Cases

## Design

This file contains a set of individuals (instances) with specific assertions designed to test the logical consistency and inferential power of the ontology. A reasoner should be able to process these without making the ontology inconsistent (except for the deliberate negative test case).

## OWL Functional Syntax

owl:functional-syntax:: |

# ========================================

# TEST INSTANCES (for validation)

# ========================================

# Test 1: Avatar should be inferred as VirtualAgent

  Declaration(NamedIndividual(mv:TestAvatar1))
  ClassAssertion(mv:Avatar mv:TestAvatar1)

# EXPECTED INFERENCES:

# ClassAssertion(mv:VirtualEntity mv:TestAvatar1)

# ClassAssertion(mv:Agent mv:TestAvatar1)

# ClassAssertion(mv:VirtualAgent mv:TestAvatar1)

# Test 2: Digital Twin must have physical and virtual bindings to be valid

  Declaration(NamedIndividual(mv:TestDT1))
  ClassAssertion(mv:DigitalTwin mv:TestDT1)
  Declaration(NamedIndividual(mv:PhysicalEngine1))
  ClassAssertion(mv:PhysicalObject mv:PhysicalEngine1)
  Declaration(NamedIndividual(mv:VirtualEngineModel1))
  ClassAssertion(mv:VirtualObject mv:VirtualEngineModel1)
  ObjectPropertyAssertion(mv:bindsTo mv:TestDT1 mv:PhysicalEngine1)
  ObjectPropertyAssertion(mv:bindsTo mv:TestDT1 mv:VirtualEngineModel1)

# EXPECTATION: This individual is consistent.

# Test 3: Inconsistent Hybrid Entity (missing a binding)

  Declaration(NamedIndividual(mv:TestInconsistentDT))
  ClassAssertion(mv:DigitalTwin mv:TestInconsistentDT)
  ObjectPropertyAssertion(mv:bindsTo mv:TestInconsistentDT mv:PhysicalEngine1)

# EXPECTATION: A reasoner should flag TestInconsistentDT as inconsistent with the HybridEntity definition.

# Test 4: Disjointness violation (should make the whole ontology inconsistent)

  Declaration(NamedIndividual(mv:TestBadEntity))
  ClassAssertion(mv:PhysicalEntity mv:TestBadEntity)
  ClassAssertion(mv:VirtualEntity mv:TestBadEntity)

# EXPECTATION: The ontology becomes inconsistent upon adding this individual.
