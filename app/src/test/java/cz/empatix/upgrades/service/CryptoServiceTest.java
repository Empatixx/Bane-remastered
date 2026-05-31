package cz.empatix.upgrades.service;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.security.Key;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Pure-logic coverage of {@link CryptoService}: AES key derivation and the encrypt/decrypt
 * round-trip. No GL/audio/asset interaction, so no static stubbing is needed.
 */
class CryptoServiceTest {

    // 16 bytes -> valid AES-128 key length.
    private static final String KEY16 = "AHTlRnmhD5QYiNvL";

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void generateKeyProducesAesKeyMatchingRawBytes() {
        final Key key = CryptoService.generateKey(KEY16);

        assertNotNull(key);
        assertEquals("AES", key.getAlgorithm());
        assertArrayEquals(KEY16.getBytes(StandardCharsets.UTF_8), key.getEncoded());
    }

    @Test
    void encryptThenDecryptRoundTripsToOriginalBytes() {
        final Key key = CryptoService.generateKey(KEY16);
        final byte[] plain = "the quick brown fox".getBytes(StandardCharsets.UTF_8);

        final byte[] cipher = CryptoService.encrypt(plain, key);
        final byte[] back = CryptoService.decrypt(cipher, key);

        assertArrayEquals(plain, back);
    }

    @Test
    void encryptChangesTheBytes() {
        final Key key = CryptoService.generateKey(KEY16);
        final byte[] plain = "0123456789abcdef".getBytes(StandardCharsets.UTF_8);

        final byte[] cipher = CryptoService.encrypt(plain, key);

        assertFalse(java.util.Arrays.equals(plain, cipher));
    }

    @Test
    void decryptWithWrongKeyThrows() {
        final Key key = CryptoService.generateKey(KEY16);
        final Key otherKey = CryptoService.generateKey("ZZZZZZZZZZZZZZZZ");
        final byte[] cipher = CryptoService.encrypt("payload".getBytes(StandardCharsets.UTF_8), key);

        // AES/ECB with PKCS5 padding: a wrong key yields a padding/block error wrapped in RuntimeException.
        assertThrows(RuntimeException.class, () -> CryptoService.decrypt(cipher, otherKey));
    }

    @Test
    void invalidKeyLengthIsWrappedInRuntimeException() {
        final Key badKey = CryptoService.generateKey("tooshort");

        assertThrows(RuntimeException.class,
                () -> CryptoService.encrypt("data".getBytes(StandardCharsets.UTF_8), badKey));
    }
}
