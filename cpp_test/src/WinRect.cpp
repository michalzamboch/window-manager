#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"

WinRect::WinRect(const RECT& rect)
{
    this->top = rect.top;
    this->bottom = rect.bottom;
    this->left = rect.left;
    this->right = rect.right;
}

std::string WinRect::toString()
{
    std::stringstream stream;
    stream 
        << "Top: " << this->top
        << " Bottom: " << this->bottom 
        << " Left: " <<  this->left 
        << " Right: " << this->right;

    return stream.str();
}

void WinRect::print()
{
    std::cout << this->toString() << std::endl;
}