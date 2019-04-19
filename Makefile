ELF_FILE := target/mips-n64-elf/release/n64-quickstart
BIN_FILE := $(ELF_FILE).bin
N64_FILE := $(ELF_FILE).n64

FS_FILE := target/mips-n64-elf/release/fs.bin

all: $(N64_FILE)

$(ELF_FILE): $(shell find src -type f)
	@echo "[XBUILD   ] $@ <- src"
	@RUSTFLAGS="-C link-arg=-Tlink.x -C lto" cargo xbuild --release --target mips-n64-elf.json

$(BIN_FILE): $(ELF_FILE)
	@echo "[OBJCOPY  ] $@ <- $^"
	@mips-unknown-elf-objcopy -O binary $^ $@

$(FS_FILE): $(shell find fs -type f )
	@echo "[RM64BUILD] $# <- $^"
	cd fs; cat index.txt | xargs cat > ../target/mips-n64-elf/release/fs.bin

$(N64_FILE): $(BIN_FILE) $(FS_FILE)
	@echo "[ROM BUILD] $@ <- $^"
	@rs64romtool build 6102-bootcode.bin 0x80001000 $(BIN_FILE) $@ $(FS_FILE)

.PHONY: clean
clean:
	@echo "[CLEAN   ] target"
	@-rm -rf target
