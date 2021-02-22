BIN_PATH := bin
OBJ_PATH := obj
SRC_PATH := src
INC_PATH := include
SGX_SDK_INC_PATH := /opt/intel/sgxsdk/include
DCAP_PATH := dcap_lib

CC := gcc
CCFLAGS := -I$(INC_PATH) -I$(SGX_SDK_INC_PATH) -I$(DCAP_PATH)
CCOBJFLAGS := $(CCFLAGS) -c

TARGET_NAME := dcap_demo
TARGET := $(BIN_PATH)/$(TARGET_NAME)

SRC := $(foreach x, $(SRC_PATH), $(wildcard $(addprefix $(x)/*,.c*)))
OBJ := $(addprefix $(OBJ_PATH)/, $(addsuffix .o, $(notdir $(basename $(SRC)))))

CLEAN_LIST := $(BIN_PATH)  $(OBJ_PATH)

.PHONY: all mkdir dcap_lib clean

default: mkdir dcap_lib all 

mkdir:
	@mkdir -p $(BIN_PATH) $(OBJ_PATH)

dcap_lib:
	make -C dcap_lib

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CCFLAGS) -o $@ $(OBJ) -Lbin -ldcap

$(OBJ_PATH)/%.o: $(SRC_PATH)/%.c*
	$(CC) -c $(CCFLAGS) $(INCLUDE) $< -o $@

clean:
	@rm -rf $(CLEAN_LIST)
