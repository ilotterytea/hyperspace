#include "star.h"

#include <math.h>
#include <stdlib.h>

#include "constants.h"
#include "raylib.h"

Vector3 Generate3DPosition() {
  double random = (double)rand() / RAND_MAX;

  double angle = random * 2.0 * M_PI;
  double radius =
      (GetScreenHeight() / SPACE_SIZE) + (random * GetScreenHeight());

  float x = radius * sin(angle);
  float y = radius * cos(angle);

  Vector3 v = {x, y, STAR_START_POS_Z};
  return v;
}

Star StarCreate() {
  return (Star){Generate3DPosition(), {1, 1}, {0, 0}, 0.25f, BLACK};
}

void StarUpdate(Star *star) {
  float screen_center_x = GetScreenWidth() / 2.0;
  float screen_center_y = GetScreenHeight() / 2.0;

  float x = star->position.x / star->position.z + screen_center_x;
  float y = star->position.y / star->position.z + screen_center_y;

  star->renderPosition.x = x;
  star->renderPosition.y = y;

  float size = (STAR_START_POS_Z - star->position.z) / (0.2 * star->position.z);

  star->size.x = size;
  star->size.y = size;
}
