#!/usr/bin/env python3
"""
Convert OWL/XML to JSON-LD format for WebVOWL
"""

from rdflib import Graph

# Load the enhanced ontology
g = Graph()
g.parse("visualization/metaverse-ontology-webvowl.owl", format="xml")

# Serialize to JSON-LD
g.serialize(destination="visualization/metaverse-ontology.jsonld", format="json-ld", indent=2)

print("✓ JSON-LD format created: visualization/metaverse-ontology.jsonld")
print(f"✓ Graph contains {len(g)} triples")
