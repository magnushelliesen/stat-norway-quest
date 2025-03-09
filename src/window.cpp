#include "window.h"
#include <SDL2/SDL.h>
#include <iostream>

// Window properties
const int WIDTH = 800;
const int HEIGHT = 600;

int window(){
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::cerr << "SDL could not initialize! SDL_Error: " << SDL_GetError() << "\n";
        return 1;
    }

    // Create window
    SDL_Window* window = SDL_CreateWindow("Cross-Platform Window", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, WIDTH, HEIGHT, SDL_WINDOW_SHOWN);
    if (!window) {
        std::cerr << "Window could not be created! SDL_Error: " << SDL_GetError() << "\n";
        SDL_Quit();
        return 1;
    }

    return 0;
}