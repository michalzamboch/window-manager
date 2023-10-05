
#include "WindowReaders.h"

namespace WindowManager
{
    BOOL CALLBACK EnumHWND(HWND hwnd, LPARAM lParam)
    {
        std::vector<HWND> &hwnds = *reinterpret_cast<std::vector<HWND> *>(lParam);
        hwnds.push_back(hwnd);

        return TRUE;
    }

    BOOL CALLBACK GetWindowsTitles(HWND hwnd, LPARAM lParam)
    {
        auto title = GetWindowTilte(hwnd);

        std::vector<std::wstring> &titles = *reinterpret_cast<std::vector<std::wstring> *>(lParam);
        titles.push_back(title);

        return TRUE;
    }

    std::wstring GetWindowTilte(HWND hwnd)
    {
        const DWORD TITLE_SIZE = 1024;
        WCHAR windowTitle[TITLE_SIZE];

        GetWindowTextW(hwnd, windowTitle, TITLE_SIZE);

        int length = ::GetWindowTextLength(hwnd);
        std::wstring title(&windowTitle[0]);

        return title;
    }

    BOOL CALLBACK GetWindowsRects(HWND hwnd, LPARAM lParam)
    {
        auto rect = GetRect(hwnd);

        WinRect winRect(rect);
        std::vector<WinRect> &rectangles = *reinterpret_cast<std::vector<WinRect> *>(lParam);
        rectangles.push_back(winRect);

        return TRUE;
    }

    RECT GetRect(HWND hwnd)
    {
        RECT rect;
        GetWindowRect(hwnd, &rect);

        return rect;
    }

    std::vector<HWND> GetAvailableHWNDs()
    {
        std::vector<HWND> hwnds;
        EnumWindows(EnumHWND, reinterpret_cast<LPARAM>(&hwnds));

        return hwnds;
    }

    std::vector<std::wstring> GetAvailableTitles()
    {
        std::vector<std::wstring> titles;
        EnumWindows(GetWindowsTitles, reinterpret_cast<LPARAM>(&titles));

        return titles;
    }

    std::vector<WinRect> GetAvailableWinRects()
    {
        std::vector<WinRect> rectangles;
        EnumWindows(GetWindowsRects, reinterpret_cast<LPARAM>(&rectangles));

        return rectangles;
    }
}