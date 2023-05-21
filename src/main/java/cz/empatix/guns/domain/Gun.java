package cz.empatix.guns.domain;

public interface Gun {
    void setShooting(final boolean shooting);
    boolean shoot(float x, float y);
    void reload();
    void update(double dt);
}
