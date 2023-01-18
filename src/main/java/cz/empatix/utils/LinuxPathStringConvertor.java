package cz.empatix.utils;

public class LinuxPathStringConvertor {
    public static String convert(final String s){
        return s.replace("\\","/");
    }
}
