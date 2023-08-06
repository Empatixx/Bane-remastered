package cz.empatix.common.initializators;

import de.jcm.discordgamesdk.Core;
import de.jcm.discordgamesdk.CreateParams;
import de.jcm.discordgamesdk.activity.Activity;
import org.apache.commons.lang3.SystemUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Path;
import java.time.Instant;

public class DiscordInit implements Initializer {
    private final static Logger LOGGER = LoggerFactory.getLogger(DiscordInit.class);
    private final static long APPLICATION_ID = 805802430553653279L;
    private final static Path LIBRARY_WIN64_PATH = Path.of("./assets/discord-api/lib/x86_64/discord_game_sdk.dll");
    private final static Path LIBRARY_WIN32_PATH = Path.of("./assets/discord-api/lib/x86/discord_game_sdk.dll");
    private final static Path LIBRARY_LINUX64_PATH = Path.of("./assets/discord-api/lib/x86_64/discord_game_sdk.so");
    private final static Path LIBRARY_MAC64_PATH = Path.of("./assets/discord-api/lib/aarch64/discord_game_sdk.so");

    @Override
    public void init() {
        try{
            LOGGER.info("Initializing Discord.");
            Core.init(selectFile().toFile());
            LOGGER.info("Discord initialized.");
            try (final CreateParams params = new CreateParams()) {
                params.setClientID(APPLICATION_ID);
                params.setFlags(CreateParams.getDefaultFlags());
                try(final Core core = new Core(params))
                {
                    try(Activity activity = new Activity())
                    {
                        activity.setDetails("Running an example");
                        activity.setState("and having fun");

                        // Setting a start time causes an "elapsed" field to appear
                        activity.timestamps().setStart(Instant.now());

                        // We are in a party with 10 out of 100 people.
                        activity.party().size().setMaxSize(100);
                        activity.party().size().setCurrentSize(10);

                        // Make a "cool" image show up
                        activity.assets().setLargeImage("test");

                        // Setting a join secret and a party ID causes an "Ask to Join" button to appear
                        activity.party().setID("Party!");
                        activity.secrets().setJoinSecret("Join!");

                        // Finally, update the current activity to our activity
                        core.activityManager().updateActivity(activity);
                    }

                    // Run callbacks forever
                    while(true)
                    {
                        core.runCallbacks();
                        try
                        {
                            // Sleep a bit to save CPU
                            Thread.sleep(16);
                        }
                        catch(InterruptedException e)
                        {
                            e.printStackTrace();
                        }
                    }
                }
            }

        } catch (UnsatisfiedLinkError | Exception e){
            LOGGER.error("Failed to initialize Discord.", e);
        }
    }
    private Path selectFile(){
        if(SystemUtils.IS_OS_WINDOWS){
            if(SystemUtils.OS_ARCH.contains("64")){
                return LIBRARY_WIN64_PATH;
            } else {
                return LIBRARY_WIN32_PATH;
            }
        } else if(SystemUtils.IS_OS_LINUX){
            return LIBRARY_LINUX64_PATH;
        } else if(SystemUtils.IS_OS_MAC){
            return LIBRARY_MAC64_PATH;
        } else {
            throw new RuntimeException("Unsupported OS.");
        }
    }

    @Override
    public void destroy() {
    }
}
