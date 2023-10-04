#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

class WinRect
{
public:
    WinRect() = default;
    WinRect(const RECT &rect);

    ~WinRect() = default;

    long top;
    long bottom;
    long left;
    long right;

    std::string toString() const;
    void print() const;
};
