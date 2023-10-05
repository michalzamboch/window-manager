
#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"

namespace WindowManager
{
    std::vector<HWND> GetAvailableHWNDs();
    std::vector<std::wstring> GetAvailableTitles();
    std::vector<WinRect> GetAvailableWinRects();
}