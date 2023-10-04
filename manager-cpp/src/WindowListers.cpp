
#include "WindowListers.h"

void ListAll()
{
    ListHWNDs();
    ListWindowTitles();
    ListWindowSizes();
}

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

    for (const auto &hwnd : hwnds)
    {
        std::cout << hwnd << std::endl;
    }
}

unsigned long WindowTitlesCount()
{
    return static_cast<unsigned long>(GetAvailableTitles().size());
}

unsigned long WindowSizesCount()
{
    return static_cast<unsigned long>(GetAvailableWinRects().size());
}

unsigned long HWNDsCount()
{
    return static_cast<unsigned long>(GetAvailableHWNDs().size());
}

void ListAllCounts()
{
    std::cout << "Titles count: " << WindowTitlesCount() << std::endl;
    std::cout << "Window sizes count: " << WindowSizesCount() << std::endl;
    std::cout << "HWND count: " << HWNDsCount() << std::endl;
}