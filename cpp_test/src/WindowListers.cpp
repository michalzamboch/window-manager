
#include "WindowListers.h"

void ListWindowTitles()
{
    auto titles = GetAvailableTitles();

    for (const auto &title : titles)
    {
        std::wcout << L"Title: " << title << std::endl;
    }
}

void ListWindowSizes()
{
    auto rectangles = GetAvailableWinRects();
    
    for (const auto &rect : rectangles)
    {
        rect.print();
    }
}

void ListHWNDs()
{
    auto hwnds = GetAvailableHWNDs();

    for (auto &hwnd : hwnds)
    {
        std::cout << hwnd << std::endl;
    }
}

void ListAll()
{
    ListHWNDs();
    ListWindowTitles();
    ListWindowSizes();
}