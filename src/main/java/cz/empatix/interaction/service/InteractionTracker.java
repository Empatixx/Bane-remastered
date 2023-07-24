package cz.empatix.interaction.service;

public class InteractionTracker {
    private final static int DEFAULT_DIFFERENCE = 100;
    private long lastInteraction;
    public void interaction(){
        lastInteraction = System.currentTimeMillis();
    }
    public boolean isInteracting(){
        return System.currentTimeMillis() - lastInteraction < DEFAULT_DIFFERENCE;
    }
}
