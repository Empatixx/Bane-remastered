package cz.empatix.hud.domain;

public class StatusController {
    private int maxValue;
    private int currentValue;
    private float elapsedTime;
    private float accumulatedDelta;

    public StatusController() {
        elapsedTime = 0;
        accumulatedDelta = 0;
    }

    public float delta(){
        return (float)currentValue / (float)maxValue;
    }

    public float timeChangeRatio(){
        if (elapsedTime == 0) {
            return 0;
        }
        return accumulatedDelta / elapsedTime;
    }

    public void update(final double dt,
                       final int currentValue,
                       final int maxValue){
        float prevDelta = delta();
        setValues(currentValue, maxValue);
        float currentDelta = delta();
        float deltaChange = Math.abs(currentDelta - prevDelta);

        accumulatedDelta += deltaChange;
        elapsedTime += dt;
    }
    private void setValues(final int currentValue,
                           final int maxValue){
        this.currentValue = currentValue;
        this.maxValue = maxValue;
    }

}
