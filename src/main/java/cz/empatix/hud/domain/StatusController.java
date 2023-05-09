package cz.empatix.hud.domain;

public class StatusController {
    private final double TWEEN = 0.05;
    private int maxValue;
    private int currentValue;
    private float delayedDelta;

    public StatusController() {
        delayedDelta = 1;
    }

    public float delta(){
        if(maxValue == 0){
            return 1;
        }
        return (float)currentValue / (float)maxValue;
    }
    public float delayedDelta(){
        return delayedDelta;
    }

    public void update(final double dt,
                       final int currentValue,
                       final int maxValue){
        delayedDelta = (delayedDelta - delta()) * (float)Math.pow(TWEEN, dt) + delta();
        this.currentValue = currentValue;
        this.maxValue = maxValue;
    }
    private void setValues(final int currentValue,
                           final int maxValue){
        this.currentValue = currentValue;
        this.maxValue = maxValue;
    }

}
