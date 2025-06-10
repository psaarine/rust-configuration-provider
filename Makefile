as_crate=--crate-name configuration_provider
target_build_dir=build
target_lib_type_dir=lib
to=--out-dir $(target_build_dir)/$(target_lib_type_dir)
crate_type=lib
as_crate_type=--crate-type=$(crate_type)
compile_string=rustc -L externals $(as_crate) $(to) $(as_crate_type)  lib.rs
cleaned_file_types=a,o,rlib

default:

	$(compile_string)

static:
	
	$(eval crate_type=staticlib)
	$(eval target_lib_type_dir=staticlib)
	$(compile_string)


clean:
