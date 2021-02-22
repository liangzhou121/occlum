CC       = gcc
CFLAGS   = -Wall -g -O2 -fPIC
CXXFLAGS =
SGX_SDK_INC_PATH := /opt/intel/sgxsdk/include
INCLUDE  = -I . -I ../include -I $(SGX_SDK_INC_PATH)
TARGET   = libdcap.so
LIBPATH  = ../bin/

vpath %.h ./inc

OBJS     = dcap.o
SRCS     = dcap.c

all: $(OBJS)
	$(CC) -shared -fPIC $(OBJS) -o $(TARGET) 
	mkdir -p $(LIBPATH)
	mv $(TARGET) $(LIBPATH)

$(OBJS): %.o: %.c
	$(CC) -c $(CFLAGS) $(INCLUDE) $< -o $@

.PHONY : clean
clean:
	rm -f *.o
	rm -f $(LIBPATH)$(TARGET)
