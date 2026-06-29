CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra
TARGET = onfex
SOURCES = main.cpp onfex.cpp

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(CXX) $(CXXFLAGS) $(SOURCES) -o $(TARGET)

run: $(TARGET)
	./$(TARGET) main.onfex

clean:
	rm -f $(TARGET)
