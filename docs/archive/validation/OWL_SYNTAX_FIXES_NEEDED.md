# OWL Syntax Errors - Comprehensive Fix List

## Summary
Found 45 instances of XSD datatype usage in VisioningLab markdown files with systematic syntax errors.

## Error Categories

### Category 1: Object Cardinality with Datatypes
**Problem:** Using `ObjectExactCardinality` or `ObjectMinCardinality` with XSD datatypes
**Fix:** Change to `DataExactCardinality` or `DataMinCardinality` (no datatype parameter needed)

**Files affected:**
1. `Non-Fungible Token (NFT).md:54` - `ObjectExactCardinality(1 mv:hasUniqueIdentifier xsd:string)`
2. `Crypto Token.md:69` - `ObjectExactCardinality(1 mv:hasTokenIdentifier xsd:string)`
3. `Cryptocurrency.md:70` - `ObjectExactCardinality(1 mv:hasCurrencySymbol xsd:string)`
4. `Fractionalized NFT.md:71` - `ObjectExactCardinality(1 mv:hasTotalSupply xsd:integer)`
5. `Liquidity Pool.md:76` - `ObjectExactCardinality(1 mv:hasInvariantFormula xsd:string)`
6. `Trust Score Metric.md:36` - `ObjectExactCardinality(1 mv:hasScoreValue xsd:decimal)`
7. `Trust Score Metric.md:39` - `ObjectExactCardinality(1 mv:hasTimestamp xsd:dateTime)`
8. `Virtual World.md:61` - `ObjectMinCardinality(1 mv:hasPersistence xsd:boolean)`
9. `Virtual World.md:64` - `ObjectMinCardinality(1 mv:hasSpatialContinuity xsd:boolean)`

**Correct Syntax:**
```owl
ObjectExactCardinality(1 mv:hasUniqueIdentifier xsd:string)  ❌
DataExactCardinality(1 mv:hasUniqueIdentifier)               ✅
```

### Category 2: ObjectSomeValuesFrom with Datatypes
**Problem:** Using `ObjectSomeValuesFrom` with XSD datatypes
**Fix:** Change to `DataSomeValuesFrom`

**Files affected:**
1. `Metaverse Architecture Stack.md:44` - `ObjectSomeValuesFrom(mv:hasLayerOrder xsd:positiveInteger)`
2. `Latency.md:60` - `ObjectSomeValuesFrom(mv:hasValue xsd:decimal)`

**Correct Syntax:**
```owl
ObjectSomeValuesFrom(mv:hasLayerOrder xsd:positiveInteger)  ❌
DataSomeValuesFrom(mv:hasLayerOrder xsd:positiveInteger)    ✅
```

### Category 3: Invalid DataRange Syntax
**Problem:** `xsd:boolean[true]` is not valid OWL 2 Functional Syntax
**Fix:** Use `DataHasValue` for specific boolean values, or `DataSomeValuesFrom` without the `[true]` suffix

**Files affected:**
1. `Glossary Index.md:68` - `DataSomeValuesFrom(mv:synchronizedWithOntology xsd:boolean[true])`
2. `Metaverse Ontology Schema.md:67` - `DataSomeValuesFrom(mv:isConsistent xsd:boolean[true])`

**Correct Syntax:**
```owl
DataSomeValuesFrom(mv:synchronizedWithOntology xsd:boolean[true])  ❌
DataHasValue(mv:synchronizedWithOntology "true"^^xsd:boolean)      ✅
```

### Category 4: Valid DataSomeValuesFrom (No Changes Needed)
**Files with correct syntax:**
1. `Algorithmic Transparency Index.md:62` - `DataSomeValuesFrom(mv:hasTransparencyScore xsd:decimal)` ✅
2. `Algorithmic Transparency Index.md:66` - `DataSomeValuesFrom(mv:hasComplianceLevel xsd:string)` ✅
3. `Algorithmic Transparency Index.md:71` - `DataSomeValuesFrom(mv:hasAssessmentDate xsd:dateTime)` ✅
4. `Threat Surface Map.md:98` - `DataSomeValuesFrom(mv:hasOverallRiskScore xsd:decimal)` ✅

**Note:** These appear correct but the validator is still failing on line 4468. Need to investigate parser output.

## Total Fixes Required
- **Category 1:** 9 files (Object cardinality → Data cardinality)
- **Category 2:** 2 files (ObjectSomeValuesFrom → DataSomeValuesFrom)
- **Category 3:** 2 files (boolean[true] → DataHasValue)
- **Total:** 13 fixes across 11 unique files

## Next Steps
1. Create automated fix script for all categories
2. Run fixes in batch
3. Re-extract complete ontology
4. Validate with zero errors
5. Proceed to WebVOWL conversion
