package kz.ilotterytea.hyperspace.world;

import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import kz.ilotterytea.hyperspace.SharedConstants;
import kz.ilotterytea.hyperspace.entities.Star;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
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
    public void render(SpriteBatch batch) {
        for (Star star : this.stars) {
            star.update();
        }

        this.stars.sort(Comparator.comparingInt(c -> (int) c.getPosition3d().z));
        Collections.reverse(this.stars);

        for (Star star : this.stars) {
            star.draw(batch);
        }
    }

    public List<Star> getStars() {
        return stars;
    }
}
