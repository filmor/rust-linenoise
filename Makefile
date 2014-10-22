INSTALL = install
PREFIX = /usr/local
INSTALL_LIB = $(PREFIX)/lib

all: src/linenoise.c src/linenoise.h
	$(CC) -Wall -W -Os -fPIC -c src/linenoise.c -o $(OUT_DIR)/linenoise.o
	$(AR) -rcs $(OUT_DIR)/liblinenoise.a $(OUT_DIR)/linenoise.o
	rm -f $(OUT_DIR)/linenoise.o

clean:
	rm -f $(OUT_DIR)/liblinenoise.a
