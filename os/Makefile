
COMPLIER = riscv64gc-unknown-none-elf

BIN_PATH = target/$(COMPLIER)/release




build: objcopy
	cargo build --target $(COMPLIER) --release

objdump:
	rust-objdump -S $(BIN_PATH)/os

readobj:
	rust-readobj -h $(BIN_PATH)/os

objcopy:
	rust-objcopy --strip-all $(BIN_PATH)/os -O binary $(BIN_PATH)/os.bin

qemu:
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=$(BIN_PATH)/os.bin,addr=0x80200000