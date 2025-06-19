CC = gcc
CFLAGS = -Wall -Wextra -std=c11
LDFLAGS = -lpthread

SRC = main.c tokens.c threads.c
OBJ = $(SRC:.c=.o)
DEPS = tokens.h threads.h

TARGET = gorillac

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c $(DEPS)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all clean
