# Cube Display Project

A simple 3D rendering project. Just a cube, points, data and distances.

Utilizes [MacroQuad](https://macroquad.rs/) for rendering to screen, matrix and vector calculations, and quaternion calculations.

Has abysmal code quality and optimization.

## Goals
Serves three goals:
- Acts as a demo and representation for a hard to illustrate 3D problem
- Acts as practice for my 3D projection skills
- Acts as practice for my Rust skills

The problem resolves around M and N. Find the minimum distance of MN given that ABCD-A'B'C'D' is a cube of side lengths 1.

## Downloads
Downloads: [Nightly](https://nightly.link/IntegerLimit/CubeDisplayProject/workflows/ci/main?preview), or just from GHA Artifacts.

## Troubleshooting
### Unable to Open on Mac (Due to Security)
1. Go to `System Settings (App)` -> `Privacy and Security`
2. Scroll Down, Find the `"X" was blocked to protect your Mac.` box, and click `Open Anyway`
3. Click `Open Anyway` in the dialog box, and enter Password / Touch ID (if requested)

### Unable to Open
Check you have the right version downloaded for your OS and Architecture!

Also, Linux ARM and Windows ARM is not supported.

### No Window Icon on Linux
Recently fixed in MiniQuad: [PR](https://github.com/not-fl3/miniquad/pull/532). Should be included in a release soon. Currently, there is no fix (unless you set an icon manually).
