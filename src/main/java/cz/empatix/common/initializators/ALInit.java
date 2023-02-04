package cz.empatix.common.initializators;

import org.lwjgl.openal.*;
import org.lwjgl.system.MemoryUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.lwjgl.openal.ALC11.*;

public class ALInit implements Initializator{
    private final static Logger LOGGER = LoggerFactory.getLogger(ALInit.class);
    private static long device;
    @Override
    public void init() {
        final String defaultDeviceName = ALC10.alcGetString(0, ALC10.ALC_DEFAULT_DEVICE_SPECIFIER);
        if(defaultDeviceName == null){
            return;
        }
        device  = alcOpenDevice(defaultDeviceName);
        if(device == MemoryUtil.NULL){
            LOGGER.error("Failed to open the default device.");
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
