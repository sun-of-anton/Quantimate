# Quantimate v0.2 — Photo-first property estimating

**Status:** Experimental Rust-first source MVP, not Xactimate feature parity. Quantimate is an independently implemented, local-first property damage estimating workspace with photo evidence, optional external vision-model recommendations, and an auditable materials/labor pricing catalog.

## Portable Windows distribution (after successful GitHub Actions build)

Users receiving a **Windows portable ZIP** do not need Rust, Cargo, Python, or a separately installed SQLite database. Download the `Quantimate-Windows-x64-portable` artifact from a successful [Windows portable build](https://github.com/sun-of-anton/Quantimate/actions/workflows/windows-portable.yml) run, extract the entire archive into a writable folder (for example, Documents), and double-click `Launch-Quantimate.cmd`. The launcher starts the local server and opens `http://127.0.0.1:8765` in the default browser. Close the Quantimate server console window to stop the app.

The app creates a `data/` folder next to the executable for its SQLite database and uploaded photos. **Back up and keep this folder** when updating to a newer app build. Each computer's data stays local: sharing an app ZIP does not sync inspections between computers. AI analysis is optional and requires the user to configure their own `OPENAI_API_KEY` environment variable. Never ship an API key in a distributable ZIP. The unsigned experimental executable may be flagged by Windows security controls; signed installers are future work. This is not a production security-reviewed or Xactimate-parity release.

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
