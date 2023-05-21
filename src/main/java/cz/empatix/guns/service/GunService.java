package cz.empatix.guns.service;

import cz.empatix.guns.domain.Gun;
import cz.empatix.guns.domain.Pistol;

import java.util.List;

public class GunService {
    private final List<Gun> guns;
    public GunService(){
        guns = List.of(
                new Pistol()
        );
    }

}
