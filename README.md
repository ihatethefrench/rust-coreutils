# coreutils
simple, portable set of unix coreutils

## compilation
- for debug builds (binaries aren't stripped or optimised)
	- `make clean debug`

- for release builds (optimised and stripped)
	- `make clean release`

- just remove `target/`
	- `make clean`

output files will be found in target/debug
