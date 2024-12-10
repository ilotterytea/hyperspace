#include "constants.h"
#include "raylib.h"
#include "star.h"
#include "stdlib.h"
#include "time.h"

int main(int argc, char *argv[]) {
  srand(time(0));

  InitWindow(800, 600, "hyperspace (demo)");

  SetTargetFPS(60);

  Star stars[STAR_AMOUNT] = {};

  for (int i = 0; i < STAR_AMOUNT; i++) {
    stars[i] = StarCreate();
  }

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(WHITE);
    DrawText("hi world!", GetScreenWidth() / 2.0 - 16 * 4,
             GetScreenHeight() / 2.0 - 16, 32, BLACK);

    for (int i = 0; i < sizeof(stars) / sizeof(stars[0]); i++) {
      Star *star = &stars[i];
      star->position.z -= star->velocity;

      if (star->position.z < 1.0) {
        star->position = Generate3DPosition();
      }

      StarUpdate(star);

      DrawRectangle(star->renderPosition.x, star->renderPosition.y,
                    star->size.x, star->size.y, star->color);
    }

    EndDrawing();
  }

  return 0;
}
