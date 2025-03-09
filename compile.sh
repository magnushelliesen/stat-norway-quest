# Compile Stat Norway Quest
g++ -Iinclude src/*.cpp -o snq `sdl2-config --cflags --libs` -lSDL2_image

# Report success or failure
if [ $? -eq 0 ]; then
    echo "Compilation successful! Run snq"
else
    echo "Compilation failed, sadly!"
fi