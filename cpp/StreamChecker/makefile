GCCFLAGS+= -std=c++17
GCCFLAGS+= -march=native
GCCFLAGS+= -Og

LDFLAGS+=  -lcurl
LDFLAGS+=  -lncurses
LDFLAGS+=  -lyajl

CPPFLAGS+= -pedantic
CPPFLAGS+= -Wall
CPPFLAGS+= -Wcast-qual
CPPFLAGS+= -Wextra
CPPFLAGS+= -Wmissing-include-dirs
CPPFLAGS+= -Wold-style-cast
CPPFLAGS+= -Wredundant-decls
CPPFLAGS+= -Wshadow
CPPFLAGS+= -Wsign-conversion
CPPFLAGS+= -Wswitch-default
CPPFLAGS+= -Werror

all:
	g++ $(GCCFLAGS) $(LDFLAGS) $(CPPFLAGS) project.cpp -o streamchecker.bin
