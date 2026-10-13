package cz.empatix.upgrades.service;

import javax.crypto.Cipher;
import javax.crypto.spec.SecretKeySpec;
import java.security.Key;

public class CryptoService {
    private static final String ALGORITHM = "AES";
    public static Key generateKey(final String key){
        return new SecretKeySpec(key.getBytes(), ALGORITHM);
    }
    public static byte[] decrypt(final byte[] serialized,final Key secretKey) {
        try {
            final Cipher cipher = Cipher.getInstance(ALGORITHM);
            cipher.init(Cipher.DECRYPT_MODE, secretKey);
            return cipher.doFinal(serialized);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
    public static byte[] encrypt(final byte[] serialized,final Key secretKey) {
        try {
            final Cipher cipher = Cipher.getInstance(ALGORITHM);
            cipher.init(Cipher.ENCRYPT_MODE, secretKey);
            return cipher.doFinal(serialized);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
