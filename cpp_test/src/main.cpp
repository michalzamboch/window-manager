#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"
#include "WindowReaders.h"

void listWindowTitles()
{
    std::vector<std::wstring> titles;
    EnumWindows(GetWindowsTitles, reinterpret_cast<LPARAM>(&titles));

    for (auto &title : titles)
        std::wcout << L"Title: " << title << std::endl;
}

void listWindowSizes()
{
    std::vector<WinRect> rectangles;
    EnumWindows(GetWindowsRects, reinterpret_cast<LPARAM>(&rectangles));
    for (const auto &rect : rectangles)
    {
        rect.print();
    }
}

void listHWNDs()
{
    auto hwnds = GetAvailableHWNDs();
    for (auto &hwnd : hwnds)
    {
        std::cout << hwnd << std::endl;
    }
}

int main()
{
    listWindowTitles();
    listWindowSizes();
    listHWNDs();

    return 0;
}