# Compile Stat Norway Quest
g++ -Iinclude src/*.cpp -o snq

# Report success or failure
if [ $? -eq 0 ]; then
    echo "Compilation successful! Run snq"
else
    echo "Compilation failed, sadly!"
fi