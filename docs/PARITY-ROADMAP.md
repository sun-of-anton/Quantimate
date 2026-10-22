# Quantimate — Xactimate feature-parity and differentiation roadmap

**Experimental v0.2 source prototype.** Do not claim Xactimate feature parity or completed photo-to-3D reconstruction.

| Capability | v0.2 status | Remaining work |
| --- | --- | --- |
| Claims, properties, rooms, evidence | Basic | Claim coverages, multiple structures, attachments, contacts, audit logs |
| Room quantity calculations | Rectangle only | Polygon rooms, openings, stairs, calibrated measurement and error bounds |
| Repair scope and manual estimating | Basic | Validated item catalog, categories, unit rules, activity libraries |
| Supplier/material cost observations | CSV import | Approved supplier feeds, freshness, inventory, tax, freight, data-rights controls |
| Installed-cost recipes | Three illustrative assemblies | Real trade-specific productivity studies, independent benchmarking, waste/finishing, equipment and labor minimums |
| Local ZIP pricing | Exact ZIP or nationwide fallback | Geographic regions, wholesale/retail segmentation, market data QA |
| AI photo interpretation | External vision endpoint suggests scope | On-device options, confidence benchmarks, segmentation and evidence-linked highlights |
| Photo-to-3D reconstruction | Not implemented | Capture guidance, SfM, depth, metric reconstruction, calibrated geometry |
| Roof sketch and takeoffs | Not implemented | Pitch, dormers, penetrations, waste and area measurements |
| Water/fire/wind/mitigation | General observations | Original loss-specific activity catalog and reviewed workflows |
| Depreciation/tax/O&P | Not implemented | RCV/ACV, policy-specific depreciation, sales tax, overhead/profit |
| Supplements/version history | Not implemented | Immutable revisions, scopes comparison, approved estimate snapshots |
| Reports | Basic JSON / browser print | Professional reports, photosheets, evidence provenance and custom templates |
| Security | Localhost only | Auth for LAN/cloud, encryption, safe photo retention and backups |
| Testing/release | Unit tests included | CI compilation, integration tests, error UX, threat model, packaging |

## Architecture principles

1. **Rust first.** Trusted source of truth for geometry, quantities, costs, snapshots and audit records; browser UI can later be wrapped with Tauri.
2. **Photo first, not CAD first.** Preserve original evidence and uncertainty. 3D presentation must not masquerade as accurate metric geometry.
3. **AI proposes; humans approve.** Never turn speculative damage recognition into automatic payable charges.
4. **Data provenance.** Every quote records supplier, rights, date, SKU and region; snapshots preserve accepted observations.
5. **Independent implementation.** Do not use or redistribute proprietary Verisk datasets or copied Xactimate line-item catalogs.
6. **Respect upstream licenses.** Material feeds, image models, geometric reconstruction libraries and AI weights may have independent restrictions.

## First meaningful milestones

- **Build gate:** cargo test and smoke-test each HTTP endpoint.
- **Inspection usability:** in-app line editing, photo notes, validation, errors, automated E2E tests.
- **Price engine:** robust units, packaging, taxes, availability, shipping and freshness; independently collected licensed supplier data.
- **Reconstruction:** guided photo intake and reproducible spatial/measurement benchmarks.
- **Estimate engine:** depreciation/RCV/ACV, O&P, waste factors and supplement revision history.
