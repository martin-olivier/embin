#include <fstream>

#include "asset.hpp"

int main()
{
    std::ofstream out("result");

    out.write(reinterpret_cast<const char *>(asset.data()), asset.size());

    return 0;
}
