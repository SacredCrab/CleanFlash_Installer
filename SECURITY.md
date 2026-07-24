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

## Dependency audit note

The .NET Framework 4.0 installer currently embeds SharpCompress 0.24.0.
NuGet reports the following moderate-severity path-traversal advisories for
that package version:

- <https://github.com/advisories/GHSA-jp7f-grcv-6mjf>
- <https://github.com/advisories/GHSA-6c8g-7p36-r338>

This repository does not treat the warnings as irrelevant. The current use is
deliberately narrower than the affected untrusted-archive scenarios:

- the installer accepts no archive path or archive bytes from the user;
- the 7z payload is generated during the pinned, checksum-verified build;
- runtime extraction skips directory entries and uses
  `ExtractFullPath = false`;
- CI tests the embedded archive and rejects Debug payload paths before
  publishing an artifact.

This limits the known traversal paths but does not make the old dependency
generally safe for other uses. Do not modify the installer to extract external
or user-supplied archives. Updating or replacing the extraction engine requires
a separate compatibility review because newer patched SharpCompress releases
no longer support the installer's .NET Framework 4.0 target.

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
