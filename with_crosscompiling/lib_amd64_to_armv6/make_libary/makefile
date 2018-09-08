CXX = arm-linux-gnueabi-gcc
CXXFLAGS= -Wall -mfloat-abi=hard

all: libtest.a

objects = test.o dep1.o dep2.o

libtest.a: $(objects)
	ar rcs libtest.a $(objects)
test.o: test.c test.h dep1.h dep2.h
	$(CXX) $(CXXFLAGS) -c test.c
dep1.o: dep1.c dep1.h dep2.h
	$(CXX) $(CXXFLAGS) -c dep1.c
dep2.o: dep2.c dep2.h
	$(CXX) $(CXXFLAGS) -c dep2.c

.PHONY: clean
clean:
	rm *.o libtest.a

