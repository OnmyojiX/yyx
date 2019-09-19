use serde_json;
use std::fs::File;
use std::time::Instant;
use yyx_types::{HeroEquip, HeroEquipAttrType, Snapshot};

const PATH: &str = "/Users/fluxxu/Projects/yyx/yyx/data/account/cbg/10_201907131401616-10-BLGOLGT3DBWAR/last_snapshot.json";

struct Attrs {
  speed: f32,
}

fn get_attrs(e: &HeroEquip) -> Attrs {
  let mut attrs = Attrs { speed: 0.0 };
  if let HeroEquipAttrType::Speed = e.base_attr.type_ {
    attrs.speed = attrs.speed + e.base_attr.value as f32;
  }
  for attr in &e.attrs {
    if attr.type_ == HeroEquipAttrType::Speed {
      attrs.speed = attrs.speed + attr.value as f32;
    }
  }
  attrs
}

fn main() {
  let f = File::open(PATH).unwrap();
  let s: Snapshot = serde_json::from_reader(f).unwrap();
  println!("total: {}", s.data.hero_equips.len());
  let attrs: Vec<_> = s.data.hero_equips.iter().map(get_attrs).collect();
  let mut by_pos: [Vec<usize>; 6] = [vec![], vec![], vec![], vec![], vec![], vec![]];
  for (i, e) in s.data.hero_equips.iter().enumerate() {
    by_pos[e.pos as usize].push(i);
  }
  for (pos, equips) in by_pos.iter().enumerate() {
    println!("[{}] {}", pos + 1, equips.len());
  }
  let c = by_pos.iter().fold(1, |c, equips| c * equips.len() as u64);
  println!("c = {}", c);

  let mut merged = Vec::with_capacity(s.data.hero_equips.len());
  let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(6);
  for p in 0..6 {
    let items = &mut by_pos[p];
    items.sort_unstable_by(|l, r| {
      attrs[*l]
        .speed
        .partial_cmp(&attrs[*r].speed)
        .unwrap()
        .reverse()
    });
    let rbegin = merged.len();
    merged.append(items);
    let rend = merged.len();
    ranges.push((rbegin, rend))
  }

  let mut c2: usize = 0;
  let mut t = Instant::now();
  let mut speed = 0.0;
  for p0 in ranges[0].0..ranges[0].1 {
    for p1 in ranges[1].0..ranges[1].1 {
      for p2 in ranges[2].0..ranges[2].1 {
        for p3 in ranges[3].0..ranges[3].1 {
          for p4 in ranges[4].0..ranges[4].1 {
            for p5 in ranges[5].0..ranges[5].1 {
              c2 = c2 + 1;
              let s = attrs[merged[p0]].speed
                + attrs[merged[p1]].speed
                + attrs[merged[p2]].speed
                + attrs[merged[p3]].speed
                + attrs[merged[p4]].speed
                + attrs[merged[p5]].speed;
              if s > speed {
                speed = s;
              }
              if c2 % 1000000000 == 0 {
                let e = Instant::now() - t;
                let ms = e.as_millis();
                t = Instant::now();
                println!("{} => {}", ms, speed);
              }
            }
          }
        }
      }
    }
  }

  println!("c2 = {}", c2);
}
