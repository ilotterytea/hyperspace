#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "hyperspace" ? 0 : 1;
}
