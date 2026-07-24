# Security policy

Adobe Flash Player is end-of-life software. This project exists only for
compatibility with legacy applications and content.

## Safe-use recommendations

- Prefer Ruffle or another maintained emulator when it supports the content.
- Use Flash only for content you trust.
- Prefer an offline, isolated virtual machine or a dedicated non-sensitive
  computer.
- Do not use an old Flash-capable browser for general web browsing.
- Verify every release with the accompanying `SHA256SUMS.txt`.
- Do not disable antivirus or Windows security protections merely to run this
  installer.

## Reporting a problem

Open a GitHub issue with:

- Windows version and 32/64-bit architecture;
- selected component type (ActiveX, NPAPI, PPAPI, or Standalone);
- exact error text;
- the diagnostic report created by `scripts/Get-FlashDiagnostics.ps1`;
- the installer SHA-256.

Remove usernames, personal paths, account names, tokens, and unrelated program
data before posting a report publicly. Do not upload proprietary SWF files or
third-party executables unless you have permission.

For a security-sensitive report, do not publish exploit details in a public
issue. Contact the repository owner through the contact method shown on their
GitHub profile.
