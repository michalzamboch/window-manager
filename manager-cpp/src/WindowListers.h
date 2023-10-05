
#include <vector>
#include <iostream>

#include "WindowReaders.h"

namespace WindowManager
{
    void ListAll();
    void ListWindowTitles();
    void ListWindowSizes();
    void ListHWNDs();

    unsigned long WindowTitlesCount();
    unsigned long WindowSizesCount();
    unsigned long HWNDsCount();
    void ListAllCounts();
}