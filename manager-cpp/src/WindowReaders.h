
#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"

std::vector<HWND> GetAvailableHWNDs();
std::vector<std::wstring> GetAvailableTitles();
std::vector<WinRect> GetAvailableWinRects();