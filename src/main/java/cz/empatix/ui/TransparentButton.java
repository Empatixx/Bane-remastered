package cz.empatix.ui;

import cz.empatix.hud.service.Text;
import cz.empatix.physics.service.IntersectionResolver;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;

import java.util.List;

public class TransparentButton implements UIElement{
    private final static Vector3i DEFAULT_COLOR = new Vector3i(222, 112, 38);
    private final static Vector3i DEFAULT_HOVER_COLOR = new Vector3i(255, 255, 0);
    private final Vector2f position;
    private final Vector2i dimensions;
    private final Text text;
    private boolean hover;
    private final int zDepth;
    private Runnable onClick;

    public TransparentButton(final String text,
                        final Vector2f position,
                        final int zDepth) {
        this.zDepth = zDepth;

        this.text = new Text(text, "PixelFJVerdana12pt.ttf", new Vector2f(), zDepth);
        this.text.setColorRGB(DEFAULT_COLOR);

        this.dimensions = new Vector2i((int)this.text.getWidth(), (int)this.text.getHeight());
        this.position = position.sub(new Vector2f(dimensions).div(2f));

        this.text.setPosition(position.x(), position.y() + dimensions.y() / 2f);
        this.text.setCenterX(position.x, position.x + dimensions.x());
    }

    @Override
    public RenderData renderData() {
        return null;
    }

    public void onClick(final Runnable onClick) {
        this.onClick = onClick;
    }

    @Override
    public List<Renderable> getSubRenderables(){
        return text.getSubRenderables();
    }
    @Override
    public boolean canRender() {
        return false;
    }

    @Override
    public int getZIndex() {
        return zDepth;
    }

    @Override
    public void update(final double dt, final double mouseX, final double mouseY) {
        hover = mouseIntersectsUI(mouseX, mouseY);
        final Vector3i color = hover ? DEFAULT_HOVER_COLOR : DEFAULT_COLOR;
        text.setColorRGB(color);
    }

    boolean mouseIntersectsUI(final double mouseX, final double mouseY) {
        return IntersectionResolver.newInstance()
                .addShape(position, new Vector2f(dimensions.x(), dimensions.y()))
                .addShape((float) mouseX, (float) mouseY)
                .resolve();
    }

    @Override
    public void mousePressed(final int button, double mouseX, double mouseY) {
        if (onClick == null) {
            return;
        }
        if (!mouseIntersectsUI(mouseX, mouseY)) {
            return;
        }
        onClick.run();
    }

    @Override
    public void mouseReleased(final int button, double mouseX, double mouseY) {

    }

    @Override
    public void keyPressed(final int key) {

    }

    @Override
    public void keyReleased(final int key) {

    }

    @Override
    public void mouseScrolled(final double xoffset, final double yoffset) {

    }

    @Override
    public boolean isHovered() {
        return hover;
    }
}
