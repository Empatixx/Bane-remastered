package cz.empatix.upgrades.service;

import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class FileUpgradeService {
    private final static Logger LOGGER = LoggerFactory.getLogger(FileUpgradeService.class);
    public Map<PlayerUpgradeName, Boolean> load(){
        LOGGER.info("Loading upgrades from file..");
        if (!Files.exists(Path.of("upgrades.bin"))) {
            createFile();
        }
        try {
            return Files.readAllLines(Path.of("upgrades.bin")).stream()
                    .map(line -> line.split("="))
                    .collect(Collectors.toMap(
                            split -> PlayerUpgradeName.valueOf(split[0]),
                            split -> Boolean.parseBoolean(split[1])
                    ));
        } catch (Exception e) {
            LOGGER.error("Error while loading file", e);
            return Stream.of(PlayerUpgradeName.values())
                    .collect(Collectors.toMap(Function.identity(), upgrade -> false));
        }
    }
    public void save(final Map<PlayerUpgradeName, Boolean> upgrades){
        LOGGER.info("Saving upgrades to file..");
        try {
            Files.write(Path.of("upgrades.bin"), upgrades.entrySet().stream()
                    .map(entry -> entry.getKey().name() + "=" + entry.getValue())
                    .collect(Collectors.toList()));
        } catch (Exception e) {
            LOGGER.error("Error while saving file", e);
        }
    }

    private void createFile(){
        LOGGER.info("Creating new file..");
        final Map<PlayerUpgradeName, Boolean> upgrades = Stream.of(PlayerUpgradeName.values())
                .collect(Collectors.toMap(Function.identity(), upgrade -> false));
        try {
            Files.write(Path.of("upgrades.bin"), upgrades.entrySet().stream()
                    .map(entry -> entry.getKey().name() + "=" + entry.getValue())
                    .collect(Collectors.toList()));
        } catch (Exception e) {
            LOGGER.error("Error while creating file", e);
        }
    }
}
