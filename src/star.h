#include "raylib.h"

typedef struct Star {
    Vector3 position;
    Vector2 size, renderPosition;
    float velocity;
    Color color;
} Star;

Star StarCreate();
void StarUpdate(Star *star, float screen_center_x, float screen_center_y);

Vector3 Generate3DPosition();
