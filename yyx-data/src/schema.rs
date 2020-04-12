table! {
    equip_group (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        name -> Text,
        equip_ids -> Text,
        created_at -> Timestamp,
    }
}

table! {
    equip_preset_ref (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        name -> Text,
        equip_ids -> Text,
        created_at -> Timestamp,
    }
}

table! {
    equip_ref (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        yys_id -> Text,
        suit_id -> Integer,
        pos -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    hero_ref (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        yys_id -> Text,
        hero_id -> Integer,
        nick_name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    player (id) {
        id -> Nullable<Integer>,
        server_id -> Integer,
        short_id -> Integer,
        cbg_server_id -> Nullable<Text>,
        cbg_order_sn -> Nullable<Text>,
        name -> Text,
        level -> Integer,
        latest_snapshot_date -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    tag (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        name -> Text,
        exclude_from_equip_calc -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    tag_object (id) {
        id -> Nullable<Integer>,
        tag_id -> Integer,
        object_type -> Integer,
        object_id -> Integer,
    }
}

joinable!(equip_group -> player (player_id));
joinable!(equip_preset_ref -> player (player_id));
joinable!(equip_ref -> player (player_id));
joinable!(hero_ref -> player (player_id));
joinable!(tag -> player (player_id));
joinable!(tag_object -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
  equip_group,
  equip_preset_ref,
  equip_ref,
  hero_ref,
  player,
  tag,
  tag_object,
);
