# Contributing

Contributions that improve compatibility, diagnostics, documentation, or
reproducibility are welcome.

## Rules

1. Keep the original MIT license and attribution.
2. Do not commit Flash binaries, installers, private test programs, account
   data, or proprietary SWF files to Git history.
3. Pin every downloaded upstream asset by both release URL and SHA-256.
4. Keep ReleaseOnly builds unable to select or install Debug components.
5. Document the Windows version, component type, file hash, and exact observed
   behavior for compatibility claims.
6. Do not describe a single-machine test as universal compatibility.

## Pull requests

Explain:

- the user-visible problem;
- the root cause or evidence;
- the files changed;
- how the change was tested;
- any compatibility or security trade-offs.

Run the release build script on Windows before requesting review:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\Build-ReleaseOnly.ps1
```
