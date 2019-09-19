use yyx_types::{HeroEquip, HeroEquipAttr, HeroEquipAttrType};

pub struct Search {
  pub suit_id_4: Option<i32>,
  pub suit_id_2: Option<i32>,
  pub pve_suit_id: Option<i32>,
  pub pos_attr_2: Option<HeroEquipAttrType>,
  pub pos_attr_4: Option<HeroEquipAttrType>,
  pub pos_attr_6: Option<HeroEquipAttrType>,
  pub targets: [Option<(Target, Range)>; 16],
  pub sort: Target,
}

pub struct Range(f32, f32);

pub enum Target {
  AttrValue(HeroEquipAttrType),
  FinalAttack,
  FinalHp,
  FinalDefense,
  CritDamage,
  CritDamageDouble,
  CritHeal,
}

pub struct Combo {
  pub suit_id_4: Option<u8>,
  pub suit_id_2: Option<u8>,
  pub demon_suit_id: Option<u8>,
  pub equips: [u16; 6],
}

#[derive(Debug)]
pub struct Equip {
  pub suit_id: i32,
  pub attrs: Attrs,
}

#[derive(Debug, Default)]
pub struct Attrs {
  pub attack: f32,
  pub attack_rate: f32,
  pub hp: f32,
  pub hp_rate: f32,
  pub defense: f32,
  pub defense_rate: f32,
  pub crit: f32,
  pub crit_rate: f32,
  pub crit_power: f32,
  pub effect_hit_rate: f32,
  pub effect_resist_rate: f32,
  pub speed: f32,
}

impl Attrs {
  pub fn get_target_value(&self, base: &Attrs, target: Target) -> f32 {
    match target {
      Target::AttrValue(t) => {
        use HeroEquipAttrType::*;
        match t {
          Hp => base.attack + self.attack,
          Defense => base.defense + self.defense,
          Attack => base.attack + self.attack,
          HpRate => base.hp_rate + self.hp_rate,
          DefenseRate => base.defense_rate + self.defense_rate,
          AttackRate => base.attack_rate + self.attack_rate,
          Speed => base.speed + self.speed,
          CritRate => base.crit_rate + self.crit_rate,
          CritPower => base.crit_power + self.crit_power,
          EffectHitRate => base.effect_hit_rate + self.effect_hit_rate,
          EffectResistRate => base.effect_resist_rate + self.effect_resist_rate,
        }
      }
      Target::FinalAttack => {
        (base.attack + self.attack) * (1.0 + base.attack_rate + self.attack_rate)
      }
      Target::FinalHp => (base.hp + self.hp) * (1.0 + base.hp_rate + self.hp_rate),
      Target::FinalDefense => {
        (base.defense + self.defense) * (1.0 + base.defense_rate + self.defense_rate)
      }
      Target::CritDamage => {
        (base.attack + self.attack)
          * (1.0 + base.attack_rate + self.attack_rate)
          * (1.0 + base.crit_power + self.crit_power)
      }
      Target::CritDamageDouble => {
        (base.attack + self.attack)
          * (1.0 + base.attack_rate + self.attack_rate)
          * (1.0 + base.crit_power + self.crit_power)
          * (1.0 + base.crit_power + self.crit_power)
      }
      Target::CritHeal => {
        (base.hp + self.hp)
          * (1.0 + base.hp_rate + self.hp_rate)
          * (1.0 + base.crit_power + self.crit_power)
      }
    }
  }
}

impl<'a> From<&'a HeroEquip> for Equip {
  fn from(e: &'a HeroEquip) -> Equip {
    let mut attrs = Attrs::default();

    fn add_attr(to: &mut Attrs, attr: &HeroEquipAttr) {
      macro_rules! match_add {
        ( $( $v:ident => $p:ident ),* ) => {
          match attr.type_ {
            $(
              HeroEquipAttrType::$v => to.$p = to.$p + attr.value as f32
            ),*
          }
        }
      }

      match_add! {
        Hp => hp,
        Defense => defense,
        Attack => attack,
        HpRate => hp_rate,
        DefenseRate => defense_rate,
        AttackRate => attack_rate,
        Speed => speed,
        CritRate => crit_rate,
        CritPower => crit_power,
        EffectHitRate => effect_hit_rate,
        EffectResistRate => effect_resist_rate
      }
    }

    add_attr(&mut attrs, &e.base_attr);
    e.attrs.iter().for_each(|attr| add_attr(&mut attrs, attr));
    e.single_attrs
      .iter()
      .for_each(|attr| add_attr(&mut attrs, attr));

    Equip {
      suit_id: e.suit_id as i32,
      attrs,
    }
  }
}
