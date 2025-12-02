#!/usr/bin/env sh

# recompile to C++ for effciency (Piet interpreters are slow)
repiet solution.png -o output.cpp --backend c++

file="output.cpp"

# piet's scaffolding is actually wrong, so we fix it
grep -q '#include <algorithm>' "$file" || sed -i '1i#include <algorithm>' "$file"
sed -i '/^void rll(int x, int y)/,/^}/c\
void rll(int x, int y) {\
 if (y<=0 || y > d.size()) return;\
 x = (y+x%y)%y;\
 if (x == 0) return;\
 std::rotate(d.end() - y, d.end() - x, d.end());\
}' "$file"

echo "Compiling..."
g++ "$file" -o output

# Let's test it with a Hello, World program!
./output < inputs/test-input.txt
