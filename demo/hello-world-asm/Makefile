clean:
	rm hello-world.o
	rm hello-world.bin

build:
	riscv64-unknown-elf-as -o hello-world.o hello-world.s
	riscv64-unknown-elf-objcopy -O binary hello-world.o hello-world.bin

info:
	riscv64-unknown-elf-objdump -d hello-world.o
