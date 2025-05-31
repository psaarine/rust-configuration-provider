as_crate = --crate-name configuration_provider
to = --out-dir build

default:
	rustc -L externals $(as_crate) $(to) lib.rs

clean:
	rm build/*.rlib
	rmdir build
