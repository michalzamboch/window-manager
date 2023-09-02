#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"
#include "WindowReaders.h"

void ListWindowTitles()
{
    std::vector<std::wstring> titles;
    EnumWindows(GetWindowsTitles, reinterpret_cast<LPARAM>(&titles));
    
    for ( auto& title : titles )
        std::wcout << L"Title: " << title << std::endl;
}

void ListWindowSizes()
{
    std::vector<WinRect> rectangles;
    EnumWindows(GetWindowsRects, reinterpret_cast<LPARAM>(&rectangles));
    for (auto& rect : rectangles)
    {
        rect.print();
    }
}

int main() {
    ListWindowTitles();
    ListWindowSizes();

    return 0;
}