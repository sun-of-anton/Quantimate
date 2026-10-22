# Quantimate v0.2 — Photo-first property estimating

**Status:** Experimental Rust-first source MVP, not Xactimate feature parity. Quantimate is an independently implemented, local-first property damage estimating workspace with photo evidence, optional external vision-model recommendations, and an auditable materials/labor pricing catalog.

## Windows end users: no Rust installation required

Once the Windows GitHub Actions job passes, download the **Quantimate-Windows-x64** artifact from the latest successful [CI run](https://github.com/sun-of-anton/Quantimate/actions/workflows/rust.yml). The workflow uploads the ready-to-extract ZIP directly (no nested archive). After a version tag is published, the same ZIP is available publicly from [GitHub Releases](https://github.com/sun-of-anton/Quantimate/releases) without requiring users to navigate CI.

Extract the portable ZIP, then **double-click `Launch-Quantimate.cmd`**. The launcher starts `Quantimate.exe` (a local Rust-powered HTTP server) and opens your default web browser at http://127.0.0.1:8765.

- Rust, Cargo, Node, Python, and admin permissions are **not required** to run a prebuilt ZIP.
- The local console window must remain open while working. Closing it stops Quantimate.
- Inspections and uploaded photos persist under `%LOCALAPPDATA%\\Quantimate` (not inside the downloaded ZIP). Back up that directory.
- This is **not yet a signed Windows installer or standalone graphical desktop app**. Microsoft SmartScreen or endpoint policy may restrict unsigned executables.
- Optional AI analysis requires a user-provided `OPENAI_API_KEY` and sends selected evidence to an external AI provider. Manual estimating is offline after installation.

See `packaging/windows/README-Windows.txt` for end-user operating instructions. To publish a version after a green build and reviewed merge, push a tag such as `v0.2.1`; GitHub Actions runs tests and publishes a prerelease download.

## Run from source (developers)



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

GitHub Actions is configured to compile, test, and smoke-test Windows portable builds. Confirm a green run before distributing an executable. It is **not production-ready**. It does *not* reconstruct 3D geometry, estimate from images automatically without human approval, support complete carrier workflows, or provide Xactimate functional parity. See `docs/PARITY-ROADMAP.md` for remaining work.

## Data, model and privacy rules

Quantimate ships with no proprietary Verisk price lists and does not scrape retailer websites. Import only datasets with reuse permission; model-provider terms apply to AI inference. Keep secrets in environment variables, not source control. Local-only operation is not equivalent to encryption or regulatory-grade data protection.

## License

Apache-2.0 (see root `LICENSE`). Supplier datasets, 3rd-party AI models and future plugins can carry separate licensing requirements.
