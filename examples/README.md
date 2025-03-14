# BSP Examples
This directory contains source files of examples provided with the Board Support
Packages (BSPs).

These are distributed to the BSP crates using the `manage` tool, which does some
simple transformation to e.g. replace `{{bsp}}` in the example names with the
specific BSP as it is copied to the BSP crate.  `manage` is configured through
`/manage.toml`, which can be used as a reference to see which BSPs support
generic examples.

## Naming
Example files are named like `<bsp group>-<example name>.rs`, where `<bsp
group>` is either a specific BSP crate name, or `generic` to indicate that
`examples.toml` will contain a list of BSPs that support the example.
