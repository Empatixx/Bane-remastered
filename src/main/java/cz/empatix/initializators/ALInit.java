package cz.empatix.initializators;

import org.lwjgl.openal.*;
import org.lwjgl.system.MemoryUtil;

import static org.lwjgl.openal.ALC11.*;

public class ALInit implements Initializator{
    private static long device;
    @Override
    public void init() {
        final String defaultDeviceName = ALC10.alcGetString(0, ALC10.ALC_DEFAULT_DEVICE_SPECIFIER);
        if(defaultDeviceName == null){
            return;
        }
        device  = alcOpenDevice(defaultDeviceName);
        if(device == MemoryUtil.NULL){
            return;
        }

        final long context = alcCreateContext(device, (int[]) null);
        alcMakeContextCurrent(context);
        final ALCCapabilities alcCapabilities = ALC.createCapabilities(device);
        AL.createCapabilities(alcCapabilities);
    }

    @Override
    public void destroy() {
        ALC.destroy();
        alcCloseDevice(device);
        AL.setCurrentProcess(null);
    }
}
