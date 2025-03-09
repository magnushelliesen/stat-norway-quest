#include "window.h"
#include <SDL2/SDL.h>
#include <iostream>

// Window properties
const int WIDTH = 1900;
const int HEIGHT = 1200;

int window(){
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::cerr << "SDL could not initialize! SDL_Error: " << SDL_GetError() << "\n";
        return 1;
    }

    // Create window
    SDL_Window* window = SDL_CreateWindow("StatNorwayQuest", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, WIDTH, HEIGHT, SDL_WINDOW_SHOWN);
    if (!window) {
        std::cerr << "Window could not be created! SDL_Error: " << SDL_GetError() << "\n";
        SDL_Quit();
        return 1;
    }

    // **Event loop to keep the window open**
    bool running = true;
    SDL_Event event;
    while (running) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {  // If the user clicks the close button
                running = false;
            }
        }
        SDL_Delay(16); // Prevents high CPU usage (approx. 60 FPS)
    }

    // Cleanup
    SDL_DestroyWindow(window);
    SDL_Quit();
    
    return 0;
}