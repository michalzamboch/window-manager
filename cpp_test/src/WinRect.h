#include <windows.h>
#include <iostream>

class WinRect
{
public:
    WinRect() = default;
    WinRect(const RECT& rect);
    
    ~WinRect() = default;

    long top;
    long bottom;
    long left;
    long right;
    
    std::string toString();
    void print();
};
