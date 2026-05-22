package cz.empatix.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class RomanNumberTest {

    @Test
    void singleSymbols() {
        assertEquals("I", RomanNumber.toRoman(1));
        assertEquals("V", RomanNumber.toRoman(5));
        assertEquals("X", RomanNumber.toRoman(10));
        assertEquals("L", RomanNumber.toRoman(50));
        assertEquals("C", RomanNumber.toRoman(100));
        assertEquals("D", RomanNumber.toRoman(500));
        assertEquals("M", RomanNumber.toRoman(1000));
    }

    @Test
    void subtractiveForms() {
        assertEquals("IV", RomanNumber.toRoman(4));
        assertEquals("IX", RomanNumber.toRoman(9));
        assertEquals("XL", RomanNumber.toRoman(40));
        assertEquals("XC", RomanNumber.toRoman(90));
        assertEquals("CD", RomanNumber.toRoman(400));
        assertEquals("CM", RomanNumber.toRoman(900));
    }

    @Test
    void additiveForms() {
        assertEquals("II", RomanNumber.toRoman(2));
        assertEquals("III", RomanNumber.toRoman(3));
        assertEquals("VI", RomanNumber.toRoman(6));
        assertEquals("VIII", RomanNumber.toRoman(8));
        assertEquals("XV", RomanNumber.toRoman(15));
        assertEquals("XX", RomanNumber.toRoman(20));
        assertEquals("XXX", RomanNumber.toRoman(30));
        assertEquals("LX", RomanNumber.toRoman(60));
        assertEquals("CCC", RomanNumber.toRoman(300));
        assertEquals("MM", RomanNumber.toRoman(2000));
        assertEquals("MMM", RomanNumber.toRoman(3000));
    }

    @Test
    void combinations() {
        assertEquals("XIV", RomanNumber.toRoman(14));
        assertEquals("XXIX", RomanNumber.toRoman(29));
        assertEquals("XLII", RomanNumber.toRoman(42));
        assertEquals("XCIX", RomanNumber.toRoman(99));
        assertEquals("CDXLIV", RomanNumber.toRoman(444));
        assertEquals("DCCCLXXXVIII", RomanNumber.toRoman(888));
        assertEquals("CMXCIX", RomanNumber.toRoman(999));
        assertEquals("MCMLXXXVII", RomanNumber.toRoman(1987));
        assertEquals("MMXXVI", RomanNumber.toRoman(2026));
        assertEquals("MMMCMXCIX", RomanNumber.toRoman(3999));
    }
}
