#include "raylib.h"

int main(int argc, char *argv[]) {
  InitWindow(800, 600, "hyperspace (demo)");

  SetTargetFPS(60);

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(WHITE);
    DrawText("hi world!", GetScreenWidth() / 2.0 - 16 * 4,
             GetScreenHeight() / 2.0 - 16, 32, BLACK);
    EndDrawing();
  }

  return 0;
}
