package kz.ilotterytea.hyperspace.screens;

import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.utils.ScreenUtils;
import kz.ilotterytea.hyperspace.HyperspaceGame;
import kz.ilotterytea.hyperspace.world.Starfield;

/**
 * Game screen.
 * @author ilotterytea
 * @version 1.0
 */
public class GameScreen implements Screen {
    private final HyperspaceGame GAME = HyperspaceGame.getInstance();
    private Starfield field;

    @Override
    public void show() {
        field = new Starfield();
    }

    @Override
    public void render(float delta) {
        ScreenUtils.clear(Color.BLACK);

        GAME.getBatch().begin();
        field.render();
        GAME.getBatch().end();
    }

    @Override
    public void resize(int width, int height) {

    }

    @Override
    public void pause() {

    }

    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void dispose() {

    }
}
