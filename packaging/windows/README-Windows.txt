QUANTIMATE — WINDOWS PORTABLE (EXPERIMENTAL MVP)
================================================
No Rust installation, compilation, administrator access, or Python required to use this prebuilt application.

1. Extract the ENTIRE Quantimate-Windows-x64.zip file to a folder you can write to.
2. Double-click Launch-Quantimate.cmd.
3. A local server console opens and your default browser opens http://127.0.0.1:8765/.
4. Keep the Quantimate console window open while using the app.
5. Close that console window to stop Quantimate.

Your inspection database and uploaded photos are stored by default in:
  %LOCALAPPDATA%\Quantimate

The desktop download contains an executable plus a CMD launcher. It is not yet
a native windowed app or Windows installer, and it has not been code signed.
Windows SmartScreen or organizational endpoint protection may warn about an
unrecognized publisher. Do not bypass organizational security restrictions.

Manual inspection and estimating do not require a cloud account. The optional
AI photo assessment sends chosen photos to the OpenAI API and needs an API key,
which must be configured by the operator; the packaged app never contains a key.
External internet service is needed for that optional AI feature.

SAFETY: This is experimental estimating software. Validate measurements, costs
and all AI proposals. The app is accessible on the local device at 127.0.0.1,
not intended for exposure over a public network.

For developers: to build from source, install Rust and run cargo test and
cargo build --release. END USERS DO NOT NEED RUST.
