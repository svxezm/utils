C = clang
TARGET = program
SOURCE = main.c

all:
	$(C) $(SOURCE) -o $(TARGET) -std=c99

run: all
	./$(TARGET) && rm -f $(TARGET)
