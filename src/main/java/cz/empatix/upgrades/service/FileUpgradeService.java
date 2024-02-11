package cz.empatix.upgrades.service;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import cz.empatix.utils.DefaultObjectMapper;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.crypto.spec.SecretKeySpec;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.Key;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class FileUpgradeService {
    private final static Logger LOGGER = LoggerFactory.getLogger(FileUpgradeService.class);
    private static final ObjectMapper MAPPER = DefaultObjectMapper.MAPPER;
    private static final Path PATH = Path.of("upgrades.bin");
    private static final Key SECRET_KEY = CryptoService.generateKey("AHTlRnmhD5QYiNvL");
    public Map<PlayerUpgradeName, Boolean> load(){
        LOGGER.info("Loading upgrades from file..");
        if (!Files.exists(PATH)) {
            createFile();
        }
        try {
            final byte[] serialized = Files.readAllBytes(PATH);
            final byte[] decrypted = CryptoService.decrypt(serialized, SECRET_KEY);
            return MAPPER.readValue(decrypted, new TypeReference<Map<PlayerUpgradeName, Boolean>>() {
            });
        } catch (Exception e) {
            LOGGER.error("Error while loading file", e);
            return Stream.of(PlayerUpgradeName.values())
                    .collect(Collectors.toMap(Function.identity(), upgrade -> false));
        }
    }
    public void save(final Map<PlayerUpgradeName, Boolean> upgrades){
        LOGGER.info("Saving upgrades to file..");
        try {
            final byte[] serialized = MAPPER.writeValueAsBytes(upgrades);
            final byte[] encrypted = CryptoService.encrypt(serialized, SECRET_KEY);
            Files.write(PATH, encrypted);
        } catch (Exception e) {
            LOGGER.error("Error while saving file", e);
        }
    }

    private void createFile(){
        LOGGER.info("Creating new file..");
        final Map<PlayerUpgradeName, Boolean> upgrades = Stream.of(PlayerUpgradeName.values())
                .collect(Collectors.toMap(Function.identity(), upgrade -> false));
        save(upgrades);
    }
}
