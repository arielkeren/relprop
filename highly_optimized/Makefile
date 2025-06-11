CC = gcc
CFLAGS = -Wall -Wextra -O3 -march=native -fopenmp -flto -Isrc -Iinclude
SRC_DIR = src
INC_DIR = include
OBJ_DIR = obj
SRC = $(wildcard $(SRC_DIR)/*.c)
OBJ = $(SRC:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
TARGET = relations

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@if not exist $(OBJ_DIR) mkdir $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@if exist $(OBJ_DIR) rmdir /S /Q $(OBJ_DIR)
	@if exist $(TARGET).exe del /Q $(TARGET).exe
	@if exist results.csv del /Q results.csv

.PHONY: all clean