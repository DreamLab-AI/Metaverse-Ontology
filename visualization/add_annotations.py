#!/usr/bin/env python3
"""
Add human-readable labels and comments to OWL ontology for WebVOWL visualization
"""

from rdflib import Graph, Literal, Namespace, URIRef
from rdflib.namespace import RDF, RDFS, OWL

# Load the ontology
g = Graph()
g.parse("visualization/metaverse-ontology.owl", format="xml")

# Define namespace
MV = Namespace("https://metaverse-ontology.org/")
g.bind("mv", MV)

# Annotations dictionary: {class_name: (label, comment)}
annotations = {
    "Portability": ("Portability", "Cross-platform migration process enabling seamless asset and identity transfer between heterogeneous virtual platforms and ecosystems"),
    "Persistence": ("Persistence", "State management process ensuring data continuity, ownership records, and contextual information across sessions, platforms, and time periods"),
    "ResilienceMetric": ("Resilience Metric", "Quantitative and qualitative measurement of system robustness, fault tolerance, recovery capabilities, and service continuity under adverse conditions"),
    "VirtualProcess": ("Virtual Process", "Digital-only process, transformation, or workflow operating in virtual environments"),
    "VirtualObject": ("Virtual Object", "Digital-only entity, artifact, or data structure existing in virtual space"),
    "InfrastructureDomain": ("Infrastructure Domain", "Core system components, foundational services, and technical infrastructure for metaverse platforms"),
    "MiddlewareLayer": ("Middleware Layer", "Integration and orchestration services between application and infrastructure layers"),
    "CrossPlatformMigration": ("Cross-Platform Migration", "Technical capability enabling movement of assets, identities, and states between different platforms"),
    "AssetTransformation": ("Asset Transformation", "Process of converting, adapting, and optimizing digital assets for different platforms and use cases"),
    "FormatConversion": ("Format Conversion", "Translation between different data formats, encodings, and representations"),
    "SemanticPreservation": ("Semantic Preservation", "Maintaining meaning, intent, and semantic relationships during transformation and migration"),
    "ValidationProtocol": ("Validation Protocol", "Verification and quality assurance mechanism ensuring correctness and compliance"),
    "InteroperabilityBridge": ("Interoperability Bridge", "Connection and translation mechanism between heterogeneous systems and protocols"),
    "StandardsCompliance": ("Standards Compliance", "Adherence to industry standards, specifications, and best practices"),
    "MetadataMapping": ("Metadata Mapping", "Translation of descriptive information and attributes between different metadata schemas"),
    "IdentityFederation": ("Identity Federation", "Cross-platform user identity management enabling single sign-on and credential sharing"),
    "FidelityMaintenance": ("Fidelity Maintenance", "Preservation of quality, accuracy, and visual/functional equivalence during transformation"),
    "BackwardCompatibility": ("Backward Compatibility", "Support for legacy formats, protocols, and versions ensuring continuity"),
    "StatefulProcess": ("Stateful Process", "Process that maintains and manages internal state information across operations"),
    "ContinuityMechanism": ("Continuity Mechanism", "System ensuring uninterrupted service delivery and data availability"),
    "DataRetentionCapability": ("Data Retention Capability", "Ability to preserve and store data over extended time periods"),
    "DurabilityGuarantee": ("Durability Guarantee", "Contractual or technical promise of data permanence and integrity"),
    "ConsistencyProtocol": ("Consistency Protocol", "Mechanism ensuring data coherence and correctness across distributed systems"),
    "RecoveryMechanism": ("Recovery Mechanism", "System for detecting failures and restoring service to operational state"),
    "DistributedStateManagement": ("Distributed State Management", "Coordination of state information across multiple nodes and geographic locations"),
    "EventualConsistency": ("Eventual Consistency", "Convergence model where systems reach consistent state over time without strong synchronization"),
    "ReplicationStrategy": ("Replication Strategy", "Method for duplicating and synchronizing data across multiple locations"),
    "SessionManagement": ("Session Management", "Handling of user interaction contexts, authentication states, and temporal boundaries"),
    "AvailabilityMeasurement": ("Availability Measurement", "Quantification of service uptime, accessibility, and operational status"),
    "RecoveryTimeMeasurement": ("Recovery Time Measurement", "Metrics quantifying duration required to restore service after failures (RTO)"),
    "FaultToleranceIndicator": ("Fault Tolerance Indicator", "Measure of system ability to continue operating despite component failures"),
    "RedundancyLevel": ("Redundancy Level", "Degree of backup systems, components, and data copies available"),
    "ReliabilityScore": ("Reliability Score", "Composite rating of overall system dependability and trustworthiness"),
    "RobustnessIndicator": ("Robustness Indicator", "Measure of system strength and stability under adverse conditions"),
    "AdaptabilityMeasure": ("Adaptability Measure", "Capacity to adjust behavior and configuration in response to changing conditions"),
    "PerformanceUnderStress": ("Performance Under Stress", "Behavior metrics during high-load, resource-constrained, or degraded conditions"),
    "GracefulDegradation": ("Graceful Degradation", "Controlled reduction of functionality while maintaining core services under strain"),
    "DisasterRecoveryReadiness": ("Disaster Recovery Readiness", "Preparedness level for responding to catastrophic failures and restoring operations"),
    "SLAComplianceIndicator": ("SLA Compliance Indicator", "Measurement of adherence to service level agreements and contractual obligations"),
    "ISO25010Aligned": ("ISO 25010 Aligned", "Conformance to ISO/IEC 25010 software quality model standards"),
}

# Add annotations to each class
for class_name, (label, comment) in annotations.items():
    class_uri = MV[class_name]
    g.add((class_uri, RDFS.label, Literal(label, lang="en")))
    g.add((class_uri, RDFS.comment, Literal(comment, lang="en")))

# Add ontology-level metadata
ontology_uri = URIRef("https://metaverse-ontology.org/")
g.add((ontology_uri, RDFS.label, Literal("Metaverse Ontology", lang="en")))
g.add((ontology_uri, RDFS.comment, Literal("Formal ontology for metaverse concepts, infrastructure, and interoperability", lang="en")))

# Serialize enhanced ontology
g.serialize(destination="visualization/metaverse-ontology-webvowl.owl", format="xml")
print("✓ Enhanced ontology created: visualization/metaverse-ontology-webvowl.owl")
print(f"✓ Added {len(annotations)} class annotations (labels + comments)")
