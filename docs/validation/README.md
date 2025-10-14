# Validation Reports

This directory contains validation reports for ontology migration batches.

## Current Reports

### Batch 1.1 (2025-10-14)
- **[BATCH_1.1_SUMMARY.md](BATCH_1.1_SUMMARY.md)** - Quick status overview
- **[BATCH_1.1_REPORT.md](BATCH_1.1_REPORT.md)** - Complete validation report (424 lines)

**Status**: 1/9 files migrated (11%)
**Completed**: Data Provenance.md (term-id 20108)
**Remaining**: 8 files require migration

## Validation Workflow

1. **Migration** - Convert legacy files to new format
2. **Self-Validation** - Agent checks against template
3. **Batch Validation** - Validator agent reviews all files
4. **Extractor Test** - Verify OWL extraction works
5. **Report Generation** - Document results and issues

## Term-ID Registry

### Batch 1.1 (20100-20108)
- 20100: Persistence (not migrated)
- 20101: State Synchronization (not migrated)
- 20102: Physics Engine (not migrated)
- 20103: Avatar Interoperability (not migrated)
- 20104: WebXR API (not migrated)
- 20105: glTF (3D File Format) (not migrated)
- 20106: Identity Federation (not migrated)
- 20107: Consent Management (not migrated)
- 20108: Data Provenance ✅

### Next Available
- **20109** - Available for Batch 1.2

## Validation Checklist

Use this checklist when validating migrated files:

- [ ] `- ### OntologyBlock` heading (level 3)
- [ ] `collapsed:: true` property set
- [ ] `metaverseOntology:: true` is FIRST property
- [ ] Unique term-id assigned
- [ ] Clear definition provided
- [ ] `owl:physicality` dimension correct
- [ ] `owl:role` dimension correct
- [ ] `owl:inferred-class` matches combination
- [ ] `belongsToDomain` assigned (at least one)
- [ ] Section IDs follow pattern
- [ ] OWL axioms in ```clojure code fence
- [ ] Minimum 3 OWL axioms present
- [ ] Wikilinks use [[Brackets]]

## Extractor Testing

### Build Extractor
```bash
cd logseq-owl-extractor
cargo build --release
```

### Test Single File
```bash
./target/release/logseq-owl-extractor \
  --input ../VisioningLab/"Data Provenance.md" \
  --output /tmp/data-provenance.ofn \
  --validate
```

### Test Full Batch
```bash
./target/release/logseq-owl-extractor \
  --input ../VisioningLab \
  --output /tmp/batch-1.1.ofn \
  --validate
```

## References

- [TEMPLATE.md](../reference/TEMPLATE.md) - Standard format template
- [FORMAT_STANDARDIZED.md](../reference/FORMAT_STANDARDIZED.md) - Complete specification
- [MIGRATION_GUIDE.md](../guides/MIGRATION_GUIDE.md) - Classification guide
- [task.md](../../task.md) - Migration task instructions
