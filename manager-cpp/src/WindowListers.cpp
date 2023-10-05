
#include "WindowListers.h"

namespace WindowManager
{
    void ListAll()
    {
        ListHWNDs();
        ListWindowTitles();
        ListWindowSizes();
    }

    void ListWindowTitles()
    {
        auto titles = WindowManager::GetAvailableTitles();

        for (const auto &title : titles)
        {
            std::wcout << L"Title: " << title << std::endl;
        }
    }

    void ListWindowSizes()
    {
        auto rectangles = WindowManager::GetAvailableWinRects();
        
        for (const auto &rect : rectangles)
        {
            rect.print();
        }
    }

    void ListHWNDs()
    {
        auto hwnds = WindowManager::GetAvailableHWNDs();

        for (const auto &hwnd : hwnds)
        {
            std::cout << hwnd << std::endl;
        }
    }

    unsigned long WindowTitlesCount()
    {
        return static_cast<unsigned long>(WindowManager::GetAvailableTitles().size());
    }

    unsigned long WindowSizesCount()
    {
        return static_cast<unsigned long>(WindowManager::GetAvailableWinRects().size());
    }

    unsigned long HWNDsCount()
    {
        return static_cast<unsigned long>(WindowManager::GetAvailableHWNDs().size());
    }

    void ListAllCounts()
    {
        std::cout << "Titles count: " << WindowTitlesCount() << std::endl;
        std::cout << "Window sizes count: " << WindowSizesCount() << std::endl;
        std::cout << "HWND count: " << HWNDsCount() << std::endl;
    }
}