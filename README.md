# verified_driver_functions

./prusti-rustc <path to verifed_driver_functions/src/lib.rs> -Pcheck_overflows=false -Pcache_path=cache.bin --crate-type=lib --cfg "prusti"

./prusti-rustc ../../Desktop/verified_driver_functions/src/lib.rs -Pcheck_overflows=false -Pcache_path=../../Desktop/verified_driver_functions/cache.bin --crate-type=lib --cfg "prusti"

Other Useful flags:
-Plog_dir
-Pwrite_smt_statistics
-Pprint_typeckd_specs
-Poptimizations
-Popt_in_verification
-Pno_verify_deps