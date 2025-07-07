# GMSH bindings
This crate contains raw C bindings to [GMSH](https://gmsh.info/) FEM mesh generator.
These bindings are generated via `bindgen` and are meant to be used for building
higher level abstractions.

# What is needed to use
Currently, only Linux is supported. The library package of GMSH (`libgmsh`, `libgmsh-dev` or similar)
should be installed.
After, simple `Cargo build` should do the trick and the bindings are generated
into the finalized lib.rs.
