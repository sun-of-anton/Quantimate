# Quantimate v0.2 — Photo-first property estimating

**Status:** Experimental Rust-first source MVP, not Xactimate feature parity. Quantimate is an independently implemented, local-first property damage estimating workspace with photo evidence, optional external vision-model recommendations, and an auditable materials/labor pricing catalog.

## Run locally

Install a recent stable Rust toolchain. From the repository root:

```sh
cargo test
cargo run --release
```

Open http://127.0.0.1:8765. SQLite data and uploaded evidence are saved under `data/` by default. Configure `QUANTIMATE_DATA` to override the data path.

## Implemented source features

- Create property inspections and rooms with manually entered dimensions.
- Upload JPEG/PNG/WebP evidence and review it locally.
- Generate optional vision AI scope suggestions through OpenAI's API (requires `OPENAI_API_KEY`); **uploaded photos are transmitted to the configured external provider** when this is used.
- Add/remove estimate scope line items (manual entry, quantities and unit prices), export JSON and print to PDF in the browser.
- Import licensed or personally collected supplier prices from CSV, with ZIP, SKU, date, supplier and rights metadata.
- Resolve matching local price observations; exact ZIP first, explicitly nationwide quotes as fallback.
- Price three illustrative drywall, paint and laminate assemblies using component inputs and labor assumptions, refusing a recipe when a required quote is missing.
- Keep applied assembly component prices and source evidence in line-item snapshots.

## Demo price observations

In the app, open **Price intelligence** and import `samples/SYNTHETIC-prices.csv`. The example quotes are fictional, use synthetic ZIP `00000`, and must never be represented as real supplier or contractor prices.

## Current limitations

This is **source-only and not compiled or tested in the current delivery environment**. It is **not production-ready**. It does *not* reconstruct 3D geometry, estimate from images automatically without human approval, support complete carrier workflows, or provide Xactimate functional parity. See `docs/PARITY-ROADMAP.md` for remaining work.

## Data, model and privacy rules

Quantimate ships with no proprietary Verisk price lists and does not scrape retailer websites. Import only datasets with reuse permission; model-provider terms apply to AI inference. Keep secrets in environment variables, not source control. Local-only operation is not equivalent to encryption or regulatory-grade data protection.

## License

Apache-2.0 (see root `LICENSE`). Supplier datasets, 3rd-party AI models and future plugins can carry separate licensing requirements.
