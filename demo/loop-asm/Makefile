clean:
	rm loop.o
	rm loop.bin

build:
	riscv64-unknown-elf-as -o loop.o loop.s
	riscv64-unknown-elf-objcopy -O binary loop.o loop.bin

info:
	riscv64-unknown-elf-objdump -d loop.o
