package kz.ilotterytea.hyperspace;

import com.badlogic.gdx.Game;

/** {@link com.badlogic.gdx.ApplicationListener} implementation shared by all platforms. */
public class HyperspaceGame extends Game {
    private static HyperspaceGame instance;

    public static HyperspaceGame getInstance() {
        return instance;
    }

    public HyperspaceGame() {
        instance = this;
    }

    @Override
    public void create() {
        setScreen(new FirstScreen());
    }
}
