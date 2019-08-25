use std::convert::TryFrom;
use yyx_types::{HeroAttr, HeroEquipAttrType, HeroRarity, SnapshotData};

use crate::types::*;

impl CbgAttrType {
  fn get_base(&self) -> f64 {
    match *self {
      CbgAttrType::Hp => 114.0,
      CbgAttrType::Defense => 5.0,
      CbgAttrType::Attack => 27.0,
      CbgAttrType::HpRate => 0.03,
      CbgAttrType::DefenseRate => 0.03,
      CbgAttrType::AttackRate => 0.03,
      CbgAttrType::Speed => 3.0,
      CbgAttrType::CritRate => 0.03,
      CbgAttrType::CritPower => 0.04,
      CbgAttrType::EffectHitRate => 0.04,
      CbgAttrType::EffectResistRate => 0.04,
    }
  }
}

impl From<CbgAttrType> for HeroEquipAttrType {
  fn from(v: CbgAttrType) -> HeroEquipAttrType {
    match v {
      CbgAttrType::Hp => HeroEquipAttrType::Hp,
      CbgAttrType::Defense => HeroEquipAttrType::Defense,
      CbgAttrType::Attack => HeroEquipAttrType::Attack,
      CbgAttrType::HpRate => HeroEquipAttrType::HpRate,
      CbgAttrType::DefenseRate => HeroEquipAttrType::DefenseRate,
      CbgAttrType::AttackRate => HeroEquipAttrType::AttackRate,
      CbgAttrType::Speed => HeroEquipAttrType::Speed,
      CbgAttrType::CritRate => HeroEquipAttrType::CritRate,
      CbgAttrType::CritPower => HeroEquipAttrType::CritPower,
      CbgAttrType::EffectHitRate => HeroEquipAttrType::EffectHitRate,
      CbgAttrType::EffectResistRate => HeroEquipAttrType::EffectResistRate,
    }
  }
}

#[derive(Debug)]
pub enum CbgConversionError {
  UnknownRarity(u8),
  AttrValueParse(std::num::ParseFloatError),
  HeroIdParse(std::num::ParseIntError),
}

impl CbgStringAttrValue {
  fn is_pecentage(&self) -> bool {
    self.0.ends_with("%")
  }

  fn parse(&self) -> Result<f64, CbgConversionError> {
    let str_val: &str = self.0.as_ref();
    if self.is_pecentage() {
      let num: &str = &str_val[..(str_val.len() - 1)];
      Ok(
        num
          .parse::<f64>()
          .map_err(CbgConversionError::AttrValueParse)?
          / 100_f64,
      )
    } else {
      Ok(
        str_val
          .parse()
          .map_err(CbgConversionError::AttrValueParse)?,
      )
    }
  }
}

impl CbgHeroRarity {
  fn parse(&self) -> Result<HeroRarity, CbgConversionError> {
    match self.0 {
      1 => Ok(HeroRarity::N),
      2 => Ok(HeroRarity::R),
      3 => Ok(HeroRarity::SR),
      4 => Ok(HeroRarity::SSR),
      5 => Ok(HeroRarity::SP),
      v => Err(CbgConversionError::UnknownRarity(v)),
    }
  }
}

impl CbgHero {
  fn get_attr(&self, ty: CbgAttrType) -> Result<HeroAttr, CbgConversionError> {
    if let Some(v) = self.attrs.get(&ty) {
      HeroAttr::try_from(v)
    } else {
      Ok(HeroAttr {
        base: 0_f64,
        add_value: 0_f64,
        add_rate: 0_f64,
        value: 0_f64,
      })
    }
  }
}

impl<'a> TryFrom<&'a CbgHeroAttr> for HeroAttr {
  type Error = CbgConversionError;

  fn try_from(value: &'a CbgHeroAttr) -> Result<Self, Self::Error> {
    let base = value.val.parse()?;
    match value.add_val.as_ref() {
      Some(val) => {
        let add_val = val.parse()?;
        let is_pecentage = value.val.is_pecentage();
        Ok(HeroAttr {
          base,
          add_value: if is_pecentage { 0.0 } else { add_val },
          add_rate: if is_pecentage { add_val } else { 0.0 },
          value: base + add_val,
        })
      }
      // no base crit power and effect rate
      None => Ok(HeroAttr {
        base: 0.0,
        add_value: 0.0,
        add_rate: 0.0,
        value: base,
      }),
    }
  }
}

impl TryFrom<CbgEquipDesc> for SnapshotData {
  type Error = CbgConversionError;

  fn try_from(value: CbgEquipDesc) -> Result<Self, Self::Error> {
    use yyx_types::*;

    let player = Player {
      id: 0,
      server_id: 0,
      name: value.player_name,
      level: value.player_level,
    };

    let currency = PlayerCurrency {
      coin: value.coin,
      jade: value.jade,
      action_point: value.action_point,
      auto_point: value.action_point,
      honor: value.honor,
      medal: value.medal,
      totem_pass: value.totem_pass,
      s_jade: value.s_jade,
      skin_token: value.skin_token,
      ..Default::default()
    };

    let mut equips = vec![];
    for (_, item) in value.equips {
      let mut base_attr = HeroEquipAttr {
        type_: HeroEquipAttrType::Attack,
        value: 0.0,
      };

      let mut random_attrs = vec![];
      for (i, (ty, val)) in item.attrs.into_iter().enumerate() {
        if i == 0 {
          base_attr.type_ = ty.into();
          base_attr.value = val.parse()?;
        } else {
          random_attrs.push(HeroEquipAttr {
            type_: ty.into(),
            value: 0.0,
          })
        }
      }

      for (ty, r) in item.rattr {
        let base = ty.get_base();
        let ty: HeroEquipAttrType = ty.into();
        match random_attrs.iter().position(|attr| attr.type_ == ty) {
          Some(pos) => {
            let p = &mut random_attrs[pos];
            p.value = p.value + (base * r);
          }
          None => random_attrs.push(HeroEquipAttr {
            type_: ty,
            value: (base * r),
          }),
        }
      }

      let mut single_attrs = vec![];
      if let Some((ty, v)) = item.single_attr {
        single_attrs.push(HeroEquipAttr {
          type_: ty.into(),
          value: v.parse()?,
        })
      }

      equips.push(HeroEquip {
        id: item.id,
        suit_id: item.suit_id,
        quality: item.quality,
        pos: item.pos - 1,
        equip_id: item.equip_id,
        level: item.level,
        born: 0,
        lock: item.lock,
        garbage: item.garbage,
        base_attr,
        attrs: random_attrs.clone(),
        random_attrs,
        random_attr_rates: vec![],
        single_attrs,
      })
    }

    let mut heroes = vec![];

    for (_, item) in value.heroes {
      if item.hero_id < 200 || item.hero_id > 600 {
        continue;
      }

      let attrs = HeroAttrs {
        max_hp: item.get_attr(CbgAttrType::Hp)?,
        speed: item.get_attr(CbgAttrType::Speed)?,
        crit_power: item.get_attr(CbgAttrType::CritPower)?,
        crit_rate: item.get_attr(CbgAttrType::CritRate)?,
        defense: item.get_attr(CbgAttrType::Defense)?,
        attack: item.get_attr(CbgAttrType::Attack)?,
        effect_hit_rate: item.get_attr(CbgAttrType::EffectHitRate)?.value,
        effect_resist_rate: item.get_attr(CbgAttrType::EffectResistRate)?.value,
      };

      let hero = Hero {
        id: item.id,
        hero_id: item.hero_id,
        equips: item.equips,
        level: item.level,
        star: item.star,
        awake: item.awake,
        exp: item.exp,
        nick_name: item.nick.unwrap_or_default(),
        born: item.born,
        lock: item.lock,
        rarity: item.rarity.parse()?,
        skills: item
          .skills
          .into_iter()
          .map(|id| HeroSkill { id, level: 1 })
          .collect(),
        attrs,
      };

      heroes.push(hero)
    }

    let mut hero_book_shards = vec![];
    for (hero_id, data) in value.hero_book_shards {
      let hero_id = hero_id.parse().map_err(CbgConversionError::HeroIdParse)?;
      hero_book_shards.push(HeroBookShard {
        hero_id,
        shards: data.num,
        books: 0,
        book_max_shards: 0,
      })
    }

    Ok(SnapshotData {
      player,
      currency,
      heroes,
      hero_equips: equips,
      hero_equip_presets: vec![],
      hero_book_shards,
      realm_cards: vec![],
      story_tasks: vec![],
    })
  }
}
