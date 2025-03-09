#include "gameplay.h"
#include "window.h"
#include <iostream>

void gameplay(){
    if (window() != 0) {
        std::cerr << "Window crashed unexpectedly, and possibly violently.";
    };
    std::cout << "\nYou just worked many many years in Statistics Norway\n\n";
}