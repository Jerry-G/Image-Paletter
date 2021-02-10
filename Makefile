CXX=g++
CFLAGS=-O2 -L/usr/X11R6/lib -lm -lpthread -lX11
OBJS=main.o CImg.o

all: CImg.o main.o main

CImg.o: CImg.cpp
	$(CXX) -o $@ -c $^

main.o: main.cpp
	$(CXX) -o $@ -c $^

main: main.cpp
	$(CXX) -o $@ $(OBJS) $(CFLAGS)

clean:
	rm -f $(OBJS) main

