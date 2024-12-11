#include "star.h"

#include <math.h>
#include <stdlib.h>

#include "constants.h"
#include "raylib.h"

Vector3 Generate3DPosition() {
  double angle_random = (double)rand() / (double)RAND_MAX;
  double radius_random = (double)rand() / (double)RAND_MAX;

  double angle = angle_random * 2.0 * M_PI;

  double radius = (GetScreenHeight() / SPACE_SIZE) +
                  (radius_random * GetScreenHeight()) * SPACE_SIZE;

  float x = radius * sin(angle);
  float y = radius * cos(angle);

  Vector3 v = {x, y, STAR_START_POS_Z};
  return v;
}

Star StarCreate() {
  double velocity =
      rand() % (STAR_MAX_VELOCITY + 1 - STAR_MIN_VELOCITY) + STAR_MIN_VELOCITY;

  velocity /= 100.0;

  return (Star){Generate3DPosition(), {1, 1}, {0, 0}, velocity, BLACK};
}

void StarUpdate(Star *star, float screen_center_x, float screen_center_y) {
  float x = star->position.x / star->position.z + screen_center_x;
  float y = star->position.y / star->position.z + screen_center_y;

  star->renderPosition.x = x;
  star->renderPosition.y = y;

  float size = (STAR_START_POS_Z - star->position.z) / (0.2 * star->position.z);

  star->size.x = size;
  star->size.y = size;
}
