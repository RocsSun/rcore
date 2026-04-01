# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

build: env $(KERNEL_BIN)

setup:

	@echo "=== Setting up rCore development environment ==="
	
	# 1. 检查并添加 target
	@if rustup target list | grep -q "$(TARGET) (installed)"; then \
		echo "✓ Target $(TARGET) already installed"; \
	else \
		echo "→ Adding target $(TARGET)..."; \
		rustup target add $(TARGET); \
	fi
	
	# 2. 检查并安装 cargo-binutils
	@if cargo install --list 2>/dev/null | grep -q "cargo-binutils"; then \
		echo "✓ cargo-binutils already installed"; \
	else \
		echo "→ Installing cargo-binutils..."; \
		cargo install cargo-binutils; \
	fi
	
	# 3. 检查并添加 rust-src 组件
	@if rustup component list | grep -q "rust-src.*installed"; then \
		echo "✓ rust-src already installed"; \
	else \
		echo "→ Adding rust-src component..."; \
		rustup component add rust-src; \
	fi
	
	# 4. 检查并添加 llvm-tools-preview 组件
	@if rustup component list | grep -q "llvm-tools.*installed"; then \
		echo "✓ llvm-tools-preview already installed"; \
	else \
		echo "→ Adding llvm-tools-preview component..."; \
		rustup component add llvm-tools-preview; \
	fi
	
	@echo "=== Environment setup complete! ==="

# 强制重新安装（不检测）
setup-force:
	rustup target add $(TARGET)
	cargo install cargo-binutils --force
	rustup component add rust-src
	rustup component add llvm-tools-preview

env: setup
# 	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
# 	cargo install cargo-binutils
# 	rustup component add rust-src
# 	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cargo build $(MODE_ARG)
	@rm src/linker.ld

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner

QEMU_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

QEMU_NAME := qemu-system-riscv64
qemu-version-check:
	@sh scripts/qemu-ver-check.sh $(QEMU_NAME)

run-inner: qemu-version-check build
	@qemu-system-riscv64 $(QEMU_ARGS)

debug: qemu-version-check build
	@tmux new-session -d \
		"qemu-system-riscv64 $(QEMU_ARGS) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

gdbserver: qemu-version-check build
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient qemu-version-check