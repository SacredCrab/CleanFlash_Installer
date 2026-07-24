# Changelog

## 34.0.0.376-release-only.1

- Updated the legacy C# installer and uninstaller version metadata from
  34.0.0.330 to 34.0.0.376.
- Updated the embedded Flash payload source to
  `darktohka/clean-flash-builds` release `v1.54`.
- Removed the Debug choice from the normal wizard flow.
- Prevented `InstallFlags.DEBUG` from being set.
- Added a defense-in-depth extraction check that rejects `-debug` payloads.
- Forced the standalone player path to `flashplayer_sa.exe`.
- Preserved the upstream full cleanup of previous Flash versions and Flash
  Center.
- Added reproducible build, checksum, diagnostic, attribution, security, and
  troubleshooting documentation.

### Validation evidence

- Confirmed the diagnosed machine had the 32-bit Debug ActiveX OCX:
  `11206f2555bb41de8a254a8820a05106d4ae17848d9e57cdace973223a3612e5`.
- Confirmed the intended 32-bit Release ActiveX OCX:
  `4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42`.
- A reference ReleaseOnly installer was reproduced byte-for-byte on two local
  builds during preparation.
- Windows 10 22H2, 32-bit ActiveX host: approximately four hours of continuous
  use without recurrence of the original access violation.
