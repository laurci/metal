current_dir := ${CURDIR}

arty-build:
	cd core/arty; make clean; make

arty-download:
	cd core/arty; make download

arty: arty-build arty_download

demo-hello-world-asm:
	cd demo/hello-world-asm; make build info
	cd core/cpu; make bin_path="${current_dir}/demo/hello-world-asm/hello-world.bin" generate

demo-loop-asm:
	cd demo/loop-asm; make build info
	cd core/cpu; make bin_path="${current_dir}/demo/loop-asm/loop.bin" generate

demo-bare:
	cd demo/bare; cargo build --release; cargo make objcopy
	cd core/cpu; make bin_path="${current_dir}/demo/bare/target/riscv32imac-unknown-none-elf/release/bare.bin" generate

demo-empty:
	cd demo/empty; cargo build --release; cargo make objcopy
	cd core/cpu; make bin_path="${current_dir}/demo/empty/target/riscv32imac-unknown-none-elf/release/empty.bin" generate

demo-wheely:
	cd demo/wheely/app; cargo build --release; cargo make objcopy
	cd core/cpu; make bin_path="${current_dir}/demo/wheely/app/target/riscv32imac-unknown-none-elf/release/wheely.bin" generate
