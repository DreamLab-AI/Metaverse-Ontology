# URI Mapping and Wikilink Conversion

## Overview

This document specifies how Logseq wikilinks (`[[Page Name]]`) are converted to formal OWL IRIs during the extraction process.

## Mapping Rules

### 1. Basic Conversion

**Logseq wikilink:** `[[Avatar]]`
**OWL IRI:** `<https://metaverse-ontology.org/Avatar>`
**Prefixed form:** `mv:Avatar`

### 2. Multi-word Concepts

**Logseq wikilink:** `[[Visual Mesh]]`
**OWL IRI:** `<https://metaverse-ontology.org/VisualMesh>`
**Prefixed form:** `mv:VisualMesh`

**Transformation:**
- Remove spaces
- Preserve PascalCase
- Keep each word capitalized

### 3. Special Characters

**Logseq wikilink:** `[[3D Rendering Engine]]`
**OWL IRI:** `<https://metaverse-ontology.org/3DRenderingEngine>`
**Prefixed form:** `mv:3DRenderingEngine`

**Rules:**
- Numbers at the beginning are preserved
- Remove special characters except alphanumerics
- Hyphens can be preserved or removed based on convention

### 4. Property Names

**Logseq property:** `has-part:: [[Visual Mesh]]`
**OWL property:** `mv:hasPart`
**OWL value:** `mv:VisualMesh`

**Transformation:**
- Property names use camelCase
- Remove hyphens and capitalize the next letter
- `has-part` → `hasPart`
- `is-part-of` → `isPartOf`

## Namespace Prefixes

All prefixes are defined in [OntologyDefinition.md](OntologyDefinition.md):

```
mv:     <https://metaverse-ontology.org/>
owl:    <http://www.w3.org/2002/07/owl#>
rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
xsd:    <http://www.w3.org/2001/XMLSchema#>
dc:     <http://purl.org/dc/elements/1.1/>
dcterms: <http://purl.org/dc/terms/>
etsi:   <https://etsi.org/ontology/>
iso:    <https://www.iso.org/ontology/>
```

## Conversion Examples

| Logseq Form | OWL IRI Form | Prefixed Form |
|-------------|--------------|---------------|
| `[[Avatar]]` | `<https://metaverse-ontology.org/Avatar>` | `mv:Avatar` |
| `[[Digital Twin]]` | `<https://metaverse-ontology.org/DigitalTwin>` | `mv:DigitalTwin` |
| `[[User Embodiment]]` | `<https://metaverse-ontology.org/UserEmbodiment>` | `mv:UserEmbodiment` |
| `[[Animation Rig]]` | `<https://metaverse-ontology.org/AnimationRig>` | `mv:AnimationRig` |
| `[[ACM + Web3D HAnim]]` | `<https://metaverse-ontology.org/ACM_Web3D_HAnim>` | `mv:ACM_Web3D_HAnim` |

## Data Property Value Conversion

### String Literals

**Logseq:** `maturity:: mature`
**OWL:** `"mature"^^xsd:string`

### Integer Values

**Logseq:** `term-id:: 20067`
**OWL:** `20067^^xsd:integer`

### Boolean Values

**Logseq:** `is-abstract:: true`
**OWL:** `"true"^^xsd:boolean`

## Implementation Notes

The Rust extractor tool should:

1. Parse Logseq properties and wikilinks
2. Apply transformation rules to convert to IRIs
3. Maintain a mapping table for consistency
4. Handle edge cases (special characters, numbers, etc.)
5. Validate that generated IRIs are legal OWL identifiers

## Reserved Names

The following names are reserved and should not be used for concepts:

- OWL keywords: `Class`, `ObjectProperty`, `DataProperty`, `Individual`, etc.
- RDF keywords: `type`, `Property`, `Resource`, etc.
- XSD types: `string`, `integer`, `boolean`, `date`, etc.

## Collision Resolution

If two different wikilinks normalize to the same IRI:

**Example:**
- `[[User Experience]]` → `mv:UserExperience`
- `[[UserExperience]]` → `mv:UserExperience`

**Resolution:**
1. Use the more common form as the canonical IRI
2. Create `owl:sameAs` assertion for the alternative
3. Document both forms in the concept's `synonyms` annotation
