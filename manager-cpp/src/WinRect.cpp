
#include "WinRect.h"

namespace WindowManager
{
    WinRect::WinRect(const RECT &rect)
    {
        this->top = rect.top;
        this->bottom = rect.bottom;
        this->left = rect.left;
        this->right = rect.right;
    }

    std::string WinRect::toString() const
    {
        std::stringstream stream;
        stream
            << "Top: " << this->top
            << " Bottom: " << this->bottom
            << " Left: " << this->left
            << " Right: " << this->right;

        return stream.str();
    }

    void WinRect::print() const
    {
        std::cout << toString() << std::endl;
    }
}