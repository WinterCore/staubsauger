CFLAGS = -std=c11 -Wall -Wextra
LDFLAGS = -Iinclude

.PHONY: clean debug all release staubsauger

all: staubsauger

clean:
	rm -rf staubsauger ./src/*.o

debug: CFLAGS += -ggdb
debug: staubsauger

release: CFLAGS += -DNDEBUG
release: staubsauger

staubsauger: src/main.c
	cc $(CFLAGS) -o staubsauger src/main.c $(LDFLAGS)
