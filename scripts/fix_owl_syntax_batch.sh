#!/bin/bash
# Batch fix all OWL syntax errors in VisioningLab markdown files

set -e
cd "$(dirname "$0")/.."

echo "=== OWL Syntax Batch Fix Script ==="
echo ""
echo "Fixes to apply:"
echo "- Category 1: ObjectExactCardinality → DataExactCardinality (10 instances)"
echo "- Category 2: ObjectSomeValuesFrom → DataSomeValuesFrom (2 instances)"
echo "- Category 3: xsd:boolean[true] → DataHasValue (2 instances)"
echo ""

# Category 1: Object Cardinality with datatypes → Data Cardinality
echo "Fixing Category 1: Object*Cardinality with datatypes..."

# Non-Fungible Token - hasUniqueIdentifier (already has correct DataHasValue for isFungible)
sed -i 's/ObjectExactCardinality(1 mv:hasUniqueIdentifier xsd:string)/DataExactCardinality(1 mv:hasUniqueIdentifier)/g' "VisioningLab/Non-Fungible Token (NFT).md"

# Crypto Token
sed -i 's/ObjectExactCardinality(1 mv:hasTokenIdentifier xsd:string)/DataExactCardinality(1 mv:hasTokenIdentifier)/g' "VisioningLab/Crypto Token.md"

# Cryptocurrency
sed -i 's/ObjectExactCardinality(1 mv:hasCurrencySymbol xsd:string)/DataExactCardinality(1 mv:hasCurrencySymbol)/g' "VisioningLab/Cryptocurrency.md"

# Fractionalized NFT
sed -i 's/ObjectExactCardinality(1 mv:hasTotalSupply xsd:integer)/DataExactCardinality(1 mv:hasTotalSupply)/g' "VisioningLab/Fractionalized NFT.md"

# Liquidity Pool
sed -i 's/ObjectExactCardinality(1 mv:hasInvariantFormula xsd:string)/DataExactCardinality(1 mv:hasInvariantFormula)/g' "VisioningLab/Liquidity Pool.md"

# Trust Score Metric - both ObjectExactCardinality instances
sed -i 's/ObjectExactCardinality(1 mv:hasScoreValue xsd:decimal)/DataExactCardinality(1 mv:hasScoreValue)/g' "VisioningLab/Trust Score Metric.md"
sed -i 's/ObjectExactCardinality(1 mv:hasTimestamp xsd:dateTime)/DataExactCardinality(1 mv:hasTimestamp)/g' "VisioningLab/Trust Score Metric.md"
sed -i 's/ObjectMaxCardinality(1 mv:hasValidityPeriod xsd:duration)/DataMaxCardinality(1 mv:hasValidityPeriod)/g' "VisioningLab/Trust Score Metric.md"

# Virtual World - both ObjectMinCardinality instances
sed -i 's/ObjectMinCardinality(1 mv:hasPersistence xsd:boolean)/DataMinCardinality(1 mv:hasPersistence)/g' "VisioningLab/Virtual World.md"
sed -i 's/ObjectMinCardinality(1 mv:hasSpatialContinuity xsd:boolean)/DataMinCardinality(1 mv:hasSpatialContinuity)/g' "VisioningLab/Virtual World.md"

echo "✓ Fixed Category 1"

# Category 2: ObjectSomeValuesFrom with datatypes → DataSomeValuesFrom
echo "Fixing Category 2: ObjectSomeValuesFrom with datatypes..."

# Metaverse Architecture Stack
sed -i 's/ObjectSomeValuesFrom(mv:hasLayerOrder xsd:positiveInteger)/DataSomeValuesFrom(mv:hasLayerOrder xsd:positiveInteger)/g' "VisioningLab/Metaverse Architecture Stack.md"

# Latency
sed -i 's/ObjectSomeValuesFrom(mv:hasValue xsd:decimal)/DataSomeValuesFrom(mv:hasValue xsd:decimal)/g' "VisioningLab/Latency.md"

echo "✓ Fixed Category 2"

# Category 3: xsd:boolean[true] → DataHasValue
echo "Fixing Category 3: Invalid boolean[true] syntax..."

# Glossary Index
sed -i 's|DataSomeValuesFrom(mv:synchronizedWithOntology xsd:boolean\[true\])|DataHasValue(mv:synchronizedWithOntology "true"^^xsd:boolean)|g' "VisioningLab/Glossary Index.md"

# Metaverse Ontology Schema
sed -i 's|DataSomeValuesFrom(mv:isConsistent xsd:boolean\[true\])|DataHasValue(mv:isConsistent "true"^^xsd:boolean)|g' "VisioningLab/Metaverse Ontology Schema.md"

echo "✓ Fixed Category 3"
echo ""
echo "=== All fixes applied successfully ==="
echo "Total files modified: 11"
echo ""
echo "Next: Run extraction to verify fixes"
