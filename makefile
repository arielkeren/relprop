CC = gcc
CFLAGS = -Wall -Wextra -O3 -march=native
SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)
TARGET = relations

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	del /Q *.o $(TARGET).exe 2>NUL || rm -f *.o $(TARGET)

.PHONY: all clean