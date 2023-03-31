#include <fstream>

#include "asset.hpp"

int main()
{
    std::ofstream out("result");

    out.write(reinterpret_cast<const char *>(bash.data()), bash.size());

    return 0;
}
