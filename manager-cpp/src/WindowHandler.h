
#include <windows.h>
#include <iostream>

#include "WinRect.h"

namespace WindowManager
{
    class WindowHandler
    {
    private:
        HWND handler;
        std::wstring title;
        WinRect rect;

    public:
        WindowHandler() = delete;
        WindowHandler(const HWND &handler);

        ~WindowHandler() = default;
    };
}