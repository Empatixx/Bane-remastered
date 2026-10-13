package cz.empatix.minimap.domain;

public enum BigMinimapRenderType {
    CURRENT(1f),
    VISITED(0.75f),
    UNVISITED(0.5f),
    UNSEEN(0f);
    private final float alpha;
    BigMinimapRenderType(final float alpha) {
        this.alpha = alpha;
    }
    public float alpha() {
        return alpha;
    }
}
