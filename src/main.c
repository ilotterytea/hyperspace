#include "constants.h"
#include "raylib.h"
#include "star.h"

int main(int argc, char *argv[]) {
  Star *stars[STAR_AMOUNT] = {};

  for (int i = 0; i < STAR_AMOUNT; i++) {
    Star star = {{0, 0, 0}, {20, 20}, 1.0, BLACK};
    stars[i] = &star;
  }

  InitWindow(800, 600, "hyperspace (demo)");

  SetTargetFPS(60);

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(WHITE);
    DrawText("hi world!", GetScreenWidth() / 2.0 - 16 * 4,
             GetScreenHeight() / 2.0 - 16, 32, BLACK);

    for (int i = 0; i < sizeof(stars) / sizeof(stars[0]); i++) {
      Star *star = stars[i];

      DrawRectangle(star->position.x, star->position.y, star->size.x,
                    star->size.y, star->color);
    }

    EndDrawing();
  }

  return 0;
}
