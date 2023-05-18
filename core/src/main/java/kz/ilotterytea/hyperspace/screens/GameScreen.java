package kz.ilotterytea.hyperspace.screens;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.*;
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
    private OrthographicCamera camera;

    @Override
    public void show() {
        field = new Starfield();
        camera = new OrthographicCamera();
        camera.setToOrtho(false, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
    }

    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(0f, 0f, 0f, 1f);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        GAME.getBatch().setProjectionMatrix(camera.combined);
        GAME.getBatch().begin();
        field.render(GAME.getBatch());
        GAME.getBatch().end();
    }

    @Override
    public void resize(int width, int height) {
        camera.setToOrtho(false, width, height);
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
