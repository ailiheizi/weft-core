# Package system v2 migration

Package system v2 changes the runtime model from package dependency management to deterministic module profiles plus additive hooks.

The target boundary is:
- weft-core owns module loading, hook dispatch, profile validation, checksums, and runtime lifecycle.
- weft-packages owns official module sources, profiles, overlays, build artifacts, and package release versions.
- weft owns the desktop client and chooses a profile for each desktop release.

The first implementation milestone is intentionally narrow:
1. Add a HookHost and versioned hook contract types beside the existing loader.
2. Add a ProfileManifest that lists exact modules and overlays.
3. Add a verified staged update path for that profile.
4. Port weft-claw as the pilot base module and one overlay.
5. Mark the old install, remote index, source precedence, and dependency-resolution endpoints as legacy. Do not delete them until the pilot ships.

v2 does not include a package solver. Core validates only explicit profile entries, core compatibility, artifact hash, and hook API compatibility. Existing capability metadata stays available to runtime routing and diagnostics.

Historical implementation is preserved in branch legacy/package-system-v1.
