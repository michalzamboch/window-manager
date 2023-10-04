
#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"

BOOL CALLBACK EnumHWND(HWND hwnd, LPARAM lParam);
BOOL CALLBACK GetWindowsTitles(HWND hwnd, LPARAM lParam);
BOOL CALLBACK GetWindowsRects(HWND hwnd, LPARAM lParam);

std::vector<HWND> GetAvailableHWNDs();
std::vector<std::wstring> GetAvailableTitles();
std::vector<WinRect> GetAvailableWinRects();