# Frontend dependencies log

If a dependency is added, please make an entry describing the issue it was added for, the date, package name, and what functionality it provides.

If a dependency is removed, please state why.

Don't worry about minor upgrades. If there's a major upgrade, consider putting something in here.

## 2025-02-25 `svelte-sonner`

[#21](https://github.com/crownshy/comhairle/issues/21)

Provides toast functionality. Installed with shadcn-sonner.

The shadcn installer tried to install `mode-watcher` as well, but I removed that as we don't have a dark mode.
