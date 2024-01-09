use binary_derive::Binary;

#[derive(Default, Debug, Binary)]
#[data(datatype = "U8")]
pub enum MobEffectOperation {
    #[variant(tag = 1)]
    Add,
    Modify,
    Remove,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
pub enum MobEffectType {
    #[variant(tag = 1)]
    Speed,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Levitation,
    FatalPoison,
    ConduitPower,
    SlowFalling,
    #[default]
    Invalid,
}
