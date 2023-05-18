package kz.ilotterytea.hyperspace.entities;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.Batch;
import com.badlogic.gdx.graphics.g2d.Sprite;
import com.badlogic.gdx.math.Vector3;
import kz.ilotterytea.hyperspace.SharedConstants;

/**
 * Star entity.
 * @author ilotterytea
 * @version 1.0
 */
public class Star extends Sprite {
    private Vector3 position3d;
    private final float velocity;

    /**
     * Star entity.
     * @author ilotterytea
     */
    public Star() {
        super(new Texture(Gdx.files.internal("sprites/star.png")));

        // Setting 3D position:
        this.position3d = get3dPosition();

        // Setting velocity:
        this.velocity = (float) (Math.random() * SharedConstants.STARS_MAX_VELOCITY);

        // Setting random color:
        super.setColor(SharedConstants.STARS_COLORS[(int) Math.round(Math.random() * (SharedConstants.STARS_COLORS.length - 1))]);
    }

    private Vector3 get3dPosition() {
        float angle = (float) (Math.random() * (2 * Math.PI));
        float radius = (float) (Math.random() * Gdx.graphics.getHeight());

        float x = (float) (radius * Math.sin(angle));
        float y = (float) (radius * Math.cos(angle));

        return new Vector3(x, y, SharedConstants.STARS_POS_Z_INIT_DISTANCE);
    }

    /**
     * Update position.
     */
    public void update() {

    }

    @Override
    public void draw(Batch batch) {
        super.draw(batch);
    }
}
