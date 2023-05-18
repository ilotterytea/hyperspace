package kz.ilotterytea.hyperspace;

import com.badlogic.gdx.Game;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;

/** {@link com.badlogic.gdx.ApplicationListener} implementation shared by all platforms. */
public class HyperspaceGame extends Game {
    private SpriteBatch batch;
    private static HyperspaceGame instance;

    public SpriteBatch getBatch() {
        return batch;
    }

    public static HyperspaceGame getInstance() {
        return instance;
    }

    public HyperspaceGame() {
        instance = this;
    }

    @Override
    public void create() {
        batch = new SpriteBatch();
        setScreen(new FirstScreen());
    }
}
