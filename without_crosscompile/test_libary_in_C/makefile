CXX = gcc
CXXFLAGS= -Wall

# Get the location of this makefile.
ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
LIB_DIR:=$(ROOT_DIR)/Clibs

all: testlib
cwd: 
	@echo $(LIB_DIR)


testlib: main.o
	$(CXX) $(CXXFLAGS) -L$(LIB_DIR)/test -o testlib main.o -ltest
main.o: main.c
	$(CXX) $(CXXFLAGS) -c main.c

.PHONY: clean
clean:
	rm *.o testlib
