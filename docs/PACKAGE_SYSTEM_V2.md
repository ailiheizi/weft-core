# Product packages and additive hooks

Package system v2 is intentionally small.

A package is a product in a directory. A product may contain local add-on packages in its addons directory. Core only loads the product, reads its exported hook names, and attaches add-ons to hooks that exist.

Core does not manage package versions, downloads, hashes, registries, profiles, dependency graphs, source priority, staged activation, or automatic updates. Updating means changing the product directory with ordinary development tooling.

The only compatibility boundary is the hook name itself. A product publishes a stable name such as weft_claw.turn.before_tools.v1. If its payload contract changes incompatibly, it publishes a new name. An add-on can therefore only affect behavior the product intentionally opened.

Implementation order:
1. Keep the small hook contract and validation module.
2. Teach package manifests to export hooks and declare one add-on attachment.
3. Make the loader scan product/addons.
4. Port weft-claw first, then remove legacy package-manager paths after the pilot works.

Historical package-manager code remains in legacy/package-system-v1.
