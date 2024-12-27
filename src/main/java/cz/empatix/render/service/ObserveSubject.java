package cz.empatix.render.service;

public interface ObserveSubject {
    void addObserver(Observer observer);
    void removeObserver(Observer observer);
    void notifyObservers(final ObservePayload payload);
}
