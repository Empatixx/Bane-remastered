package cz.empatix.common.utils;

public interface Task {
    void update(float dt);
    void start();
    void reset();
    void stop();
}
