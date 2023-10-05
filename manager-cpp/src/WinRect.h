#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

namespace WindowManager
{
    class WinRect
    {
    public:
        WinRect() = delete;
        WinRect(const RECT &rect);

        ~WinRect() = default;

        long top;
        long bottom;
        long left;
        long right;

        std::string toString() const;
        void print() const;
    };
}