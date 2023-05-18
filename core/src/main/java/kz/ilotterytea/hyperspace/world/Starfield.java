package kz.ilotterytea.hyperspace.world;

import kz.ilotterytea.hyperspace.HyperspaceGame;
import kz.ilotterytea.hyperspace.SharedConstants;
import kz.ilotterytea.hyperspace.entities.Star;

import java.util.ArrayList;
import java.util.List;

/**
 * Star holder.
 * @author ilotterytea
 * @version 1.0
 */
public class Starfield {
    /**
     * Created stars.
     */
    private final List<Star> stars;

    /**
     * Star holder.
     * @author ilotterytea
     */
    public Starfield() {
        this.stars = new ArrayList<>();

        for (int i = 0; i < SharedConstants.NUM_STARS; i++) {
            this.stars.add(new Star());
        }
    }

    /**
     * Update and render stars.
     */
    public void render() {
        for (Star star : this.stars) {
            star.update();
        }

        for (Star star : this.stars) {
            star.draw(HyperspaceGame.getInstance().getBatch());
        }
    }

    public List<Star> getStars() {
        return stars;
    }
}
