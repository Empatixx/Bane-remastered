package cz.empatix.upgrades.service;

import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import cz.empatix.upgrades.domain.QualityLevel;
import cz.empatix.upgrades.domain.TreeType;

import java.util.List;

public class UpgradeTreeFactory {
    public static UpgradeTree createUtilityTree() {
        final PlayerUpgrade improvedMovement = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, true);
        improvedMovement.setTitle("Swiftwind Steps");
        improvedMovement.setDescription("Increases movement speed\n by 10%");
        improvedMovement.setPosition(364, 125);
        improvedMovement.setPrice(20);
        improvedMovement.setName(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED);

        final PlayerUpgrade discount = new PlayerUpgrade("ingame/talents/discount.tga", QualityLevel.COMMON, true);
        discount.setTitle("Discount");
        discount.setDescription("The shop prices will\n have a chance to\n be reduced");
        discount.setPosition(364, 199);
        discount.setPrice(60);
        discount.setParent(improvedMovement);
        discount.setName(PlayerUpgradeName.DISCOUNT);

        final PlayerUpgrade trap = new PlayerUpgrade("ingame/talents/trap.tga", QualityLevel.RARE, false);
        trap.setTitle("Trap Master");
        trap.setDescription("All traps have a 30% chance\n to be immobilized");
        trap.setPosition(364, 273);
        trap.setPrice(140);
        trap.setParent(discount);
        trap.setName(PlayerUpgradeName.TRAP_MASTER);


        final PlayerUpgrade additionalGun = new PlayerUpgrade("ingame/talents/extragun.tga", QualityLevel.EPIC, false);
        additionalGun.setTitle("Additional Gun");
        additionalGun.setDescription("Adds an additional slot\n for a weapon");
        additionalGun.setPosition(564, 347);
        additionalGun.setPrice(300);
        additionalGun.setParent(trap);
        additionalGun.setName(PlayerUpgradeName.ADDITIONAL_GUN);

        final PlayerUpgrade teleportation = new PlayerUpgrade("ingame/talents/teleportation.tga", QualityLevel.COMMON, false);
        teleportation.setTitle("Teleportation");
        teleportation.setDescription("Teleports you to\n choosen and discovered\n room from the minimap");
        teleportation.setPosition(364, 421);
        teleportation.setPrice(500);
        teleportation.setParent(additionalGun);
        teleportation.setName(PlayerUpgradeName.TELEPORTATION);

        final UpgradeTree utilityTree = new UpgradeTree(List.of(improvedMovement, discount, trap, additionalGun, teleportation), TreeType.UTILITY);
        utilityTree.setPosition(364, 125);

        return utilityTree;
    }
    public static UpgradeTree createDefenseTree(){
        final PlayerUpgrade improvedHealth = new PlayerUpgrade("ingame/talents/health_cross.tga", QualityLevel.COMMON, true);
        improvedHealth.setTitle("Hero's Heart");
        improvedHealth.setDescription("Increases initial health\n by 20%");
        improvedHealth.setPosition(564, 125);
        improvedHealth.setPrice(20);
        improvedHealth.setName(PlayerUpgradeName.IMPROVED_HEALTH);

        final PlayerUpgrade improvedArmor = new PlayerUpgrade("ingame/talents/armor.tga", QualityLevel.COMMON, true);
        improvedArmor.setTitle("Improved Armor");
        improvedArmor.setDescription("Increases initial armor\n by 33%");
        improvedArmor.setPosition(564, 199);
        improvedArmor.setPrice(60);
        improvedArmor.setParent(improvedHealth);
        improvedArmor.setName(PlayerUpgradeName.IMPROVED_ARMOR);

        final PlayerUpgrade armorRegen = new PlayerUpgrade("ingame/talents/armorregen.tga", QualityLevel.RARE, false);
        armorRegen.setTitle("Armor Regeneration");
        armorRegen.setDescription("Restores armor after\n successfully\n clearing a room");
        armorRegen.setPosition(564, 273);
        armorRegen.setPrice(140);
        armorRegen.setParent(improvedArmor);
        armorRegen.setName(PlayerUpgradeName.ARMOR_REGENERATION);


        final PlayerUpgrade enhancedHealth = new PlayerUpgrade("ingame/talents/health_cross2.tga", QualityLevel.EPIC, false);
        enhancedHealth.setTitle("Titan's Heart");
        enhancedHealth.setDescription("Increases initial health\n by 25%");
        enhancedHealth.setPosition(364, 347);
        enhancedHealth.setPrice(300);
        enhancedHealth.setParent(armorRegen);
        enhancedHealth.setName(PlayerUpgradeName.ENHANCED_HEALTH);

        final PlayerUpgrade kevlar = new PlayerUpgrade("ingame/talents/kevlar.tga", QualityLevel.COMMON, false);
        kevlar.setTitle("Kevlar");
        kevlar.setDescription("Reduces damage taken\n by 10%");
        kevlar.setPosition(564, 421);
        kevlar.setPrice(500);
        kevlar.setParent(enhancedHealth);
        kevlar.setName(PlayerUpgradeName.REDUCED_DAMAGE_TAKEN);

        final UpgradeTree defenseTree = new UpgradeTree(List.of(improvedHealth, improvedArmor, armorRegen, enhancedHealth, kevlar), TreeType.DEFENSIVE);
        defenseTree.setPosition(564, 125);

        return defenseTree;
    }
    public static UpgradeTree createOffenseTree(){
        final PlayerUpgrade improvedDamage = new PlayerUpgrade("ingame/talents/bloodtear.tga", QualityLevel.COMMON, true);
        improvedDamage.setTitle("Sharpened Bullets");
        improvedDamage.setDescription("Increases all damage\n by 20%");
        improvedDamage.setPosition(464, 125);
        improvedDamage.setPrice(20);
        improvedDamage.setName(PlayerUpgradeName.IMPROVED_DAMAGE);

        final PlayerUpgrade additionalAmmo = new PlayerUpgrade("ingame/talents/ammo.tga", QualityLevel.COMMON, true);
        additionalAmmo.setTitle("Additional Ammo");
        additionalAmmo.setDescription("Increases ammo capacity\n by 10%");
        additionalAmmo.setPosition(464, 199);
        additionalAmmo.setPrice(60);
        additionalAmmo.setParent(improvedDamage);
        additionalAmmo.setName(PlayerUpgradeName.ADDITIONAL_AMMO);

        final PlayerUpgrade criticalHit = new PlayerUpgrade("ingame/talents/criticalhit.tga", QualityLevel.RARE, false);
        criticalHit.setTitle("Critical Hit");
        criticalHit.setDescription("Your attacks have a 10%\n chance to deal double\n damage");
        criticalHit.setPosition(464, 273);
        criticalHit.setPrice(140);
        criticalHit.setParent(additionalAmmo);
        criticalHit.setName(PlayerUpgradeName.CRITICAL_HIT);


        final PlayerUpgrade enhancedDamage = new PlayerUpgrade("ingame/talents/bloodtear2.tga", QualityLevel.EPIC, false);
        enhancedDamage.setTitle("Dangerous Bullets");
        enhancedDamage.setDescription("Increases all damage\n by 25%");
        enhancedDamage.setPosition(464, 347);
        enhancedDamage.setPrice(300);
        enhancedDamage.setParent(criticalHit);
        enhancedDamage.setName(PlayerUpgradeName.ENHANCED_DAMAGE);


        final PlayerUpgrade pet = new PlayerUpgrade("ingame/talents/pet.tga", QualityLevel.COMMON, false);
        pet.setTitle("Pet");
        pet.setDescription("Unlocks a pet that will\n help you in battle\n");
        pet.setPosition(464, 421);
        pet.setPrice(500);
        pet.setParent(enhancedDamage);
        pet.setName(PlayerUpgradeName.PET);

        final UpgradeTree offenseTree = new UpgradeTree(List.of(improvedDamage, additionalAmmo, criticalHit, enhancedDamage, pet), TreeType.OFFENSIVE);
        offenseTree.setPosition(464, 125);

        return offenseTree;
    }
}
