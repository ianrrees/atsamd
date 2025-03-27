# BSP Examples
This directory contains source files of examples provided with the Board Support
Packages (BSPs).

These are distributed to the BSP crates using the `manage` tool, which does some
simple transformation to e.g. replace `{{bsp}}` in the example names with the
specific BSP as it is copied to the BSP crate.  `manage` is configured through
`/manage.toml`, which can be used as a reference to see which BSPs support
generic examples.

Example files are named like `<bsp group>-<example name>.rs`, where `<bsp
group>` is either a specific BSP crate name, or an identifier for a group of
BSPs.  When `<bsp group>` is a specific BSP, the example file is simply renamed
to `<example name>` and copied to that BSP's example directory.  Otherwise, the
example file is processed as a template, renamed, and copied in to one or more
BSPs according to an entry in `/manage.toml`.

## Modifying Examples
After modifying source in this directory, run `cargo run example disribute` from
the workspace root directory, then check results in applicable BSP(s).

Before opening a Pull Request for any changes to examples, please ensure that
`cargo run example distribute` has been used to update the generated examples.