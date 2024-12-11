#include <stdio.h>
#include <stdlib.h>

#include "constants.h"
#include "math.h"
#include "raylib.h"
#include "star.h"
#include "stdlib.h"
#include "time.h"

int CompareStarsByZPosition(const void* a, const void* b) {
  return ((struct Star*)b)->position.z - ((struct Star*)a)->position.z;
}

int main(int argc, char* argv[]) {
  bool mouse_control = false;
  srand(time(0));

  InitWindow(800, 600, "hyperspace (demo)");

  SetTargetFPS(60);

  Star stars[STAR_AMOUNT] = {};

  Texture2D* texture = NULL;

  if (FileExists("star.png")) {
    Image image = LoadImage("star.png");
    Texture2D loadedTexture = LoadTextureFromImage(image);
    texture = &loadedTexture;
    UnloadImage(image);
  }

  for (int i = 0; i < STAR_AMOUNT; i++) {
    stars[i] = StarCreate(texture);
  }

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    DrawText("hi world!", GetScreenWidth() / 2.0 - 16 * 4,
             GetScreenHeight() / 2.0 - 16, 32, BLACK);

    qsort(stars, sizeof(stars) / sizeof(stars[0]), sizeof(struct Star),
          CompareStarsByZPosition);

    for (int i = 0; i < sizeof(stars) / sizeof(stars[0]); i++) {
      Star* star = &stars[i];
      star->position.z -= star->velocity;

      if (star->position.z < 1.0) {
        star->color.a = 0;
        star->position = Generate3DPosition();
      }

      float screen_center_x =
          mouse_control ? GetMouseX() : GetScreenWidth() / 2.0;
      float screen_center_y =
          mouse_control ? GetMouseY() : GetScreenHeight() / 2.0;

      if (star->position.z <= 5.0) {
        int x = abs((int)pow(star->renderPosition.x, star->position.z / 10.0));

        if (star->renderPosition.x < screen_center_x) star->position.x -= x;
        if (star->renderPosition.x > screen_center_x) star->position.x += x;
      }

      StarUpdate(star, screen_center_x, screen_center_y);

      star->color.a =
          255 / (star->position.z - 5 < 1 ? 1 : star->position.z - 5);

      if (star->texture == NULL) {
        DrawRectangle(star->renderPosition.x, star->renderPosition.y,
                      star->size.x, star->size.y, star->color);
      } else {
        DrawTextureEx(*star->texture, star->renderPosition, 0.0,
                      star->size.x / 10.0, star->color);
      }
    }

    EndDrawing();

    // Listening for input
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
      mouse_control = !mouse_control;
      if (mouse_control)
        HideCursor();
      else
        ShowCursor();
    }
  }

  return 0;
}
