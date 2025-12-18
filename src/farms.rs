use serde::{
  Deserialize,
  Serialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Farms {
  #[serde(rename = "farm")]
  pub farms: Vec<Farm>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Farm {
  #[serde(rename = "@farmId")]
  pub farm_id:    u8,
  #[serde(rename = "@name")]
  pub name:       Box<str>,
  #[serde(rename = "@color")]
  pub color:      u8,
  #[serde(rename = "@loan")]
  pub loan:       f32,
  #[serde(rename = "@money")]
  pub money:      f32,
  pub players:    Players,
  pub statistics: FarmStatistics,
  pub finances:   Finances
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finances {
  pub stats: Vec<Stats>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
  #[serde(rename = "player")]
  pub players: Vec<FarmPlayer>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmPlayer {
  #[serde(rename = "@uniqueUserId")]
  pub unique_user_id:      Box<str>,
  #[serde(rename = "@farmManager")]
  pub farm_manager:        bool,
  #[serde(rename = "@lastNickname")]
  pub last_nickname:       Box<str>,
  #[serde(rename = "@timeLastConnected")]
  pub time_last_connected: Box<str>,
  #[serde(rename = "@buyVehicle")]
  pub buy_vehicle:         bool,
  #[serde(rename = "@sellVehicle")]
  pub sell_vehicle:        bool,
  #[serde(rename = "@buyPlaceable")]
  pub buy_placeable:       bool,
  #[serde(rename = "@sellPlaceable")]
  pub sell_placeable:      bool,
  #[serde(rename = "@manageContracts")]
  pub manage_contracts:    bool,
  #[serde(rename = "@tradeAnimals")]
  pub trade_animals:       bool,
  #[serde(rename = "@createFields")]
  pub create_fields:       bool,
  #[serde(rename = "@landscaping")]
  pub landscaping:         bool,
  #[serde(rename = "@hireAssistant")]
  pub hire_assistant:      bool,
  #[serde(rename = "@resetVehicle")]
  pub reset_vehicle:       bool,
  #[serde(rename = "@manageProductions")]
  pub manage_productions:  bool,
  #[serde(rename = "@cutTrees")]
  pub cut_trees:           bool,
  #[serde(rename = "@manageRights")]
  pub manage_rights:       bool,
  #[serde(rename = "@transferMoney")]
  pub transfer_money:      bool,
  #[serde(rename = "@updateFarm")]
  pub update_farm:         bool,
  #[serde(rename = "@manageContracting")]
  pub manage_contracting:  bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmStatistics {
  pub traveled_distance:         f32,
  pub fuel_usage:                f32,
  pub seed_usage:                f32,
  pub spray_usage:               f32,
  pub worked_hectares:           f32,
  pub cultivated_hectares:       f32,
  pub sown_hectares:             f32,
  pub sprayed_hectares:          f32,
  pub threshed_hectares:         f32,
  pub plowed_hectares:           f32,
  pub harvested_grapes:          f32,
  pub harvested_olives:          f32,
  pub worked_time:               f32,
  pub cultivated_time:           f32,
  pub sown_time:                 f32,
  pub sprayed_time:              f32,
  pub threshed_time:             f32,
  pub plowed_time:               f32,
  pub bale_count:                u32,
  pub breed_cows_count:          u32,
  pub breed_sheep_count:         u32,
  pub breed_pigs_count:          u32,
  pub breed_chicken_count:       u32,
  pub breed_horses_count:        u32,
  #[cfg(feature = "fs25")]
  pub breed_goats_count:         u32,
  #[cfg(feature = "fs25")]
  pub breed_water_buffalo_count: u32,
  pub mission_count:             u32,
  pub revenue:                   f32,
  pub expenses:                  f32,
  pub play_time:                 f32,
  pub planted_tree_count:        u32,
  pub cut_tree_count:            u32,
  pub wood_tons_sold:            f32,
  pub tree_types_cut:            Box<str>,
  pub pet_dog_count:             u32,
  pub repair_vehicle_count:      u32,
  pub repaint_vehicle_count:     u32,
  pub horse_jump_count:          u32,
  pub sold_cotton_bales:         u32,
  pub wrapped_bales:             u32,
  pub tractor_distance:          f32,
  pub car_distance:              f32,
  pub truck_distance:            f32,
  pub horse_distance:            f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
  #[serde(rename = "@day")]
  pub day:                  u8,
  pub new_vehicles_cost:    f32,
  pub sold_vehicles:        f32,
  pub new_handtools_cost:   f32,
  pub sold_handtools:       f32,
  pub new_animals_cost:     f32,
  pub sold_animals:         f32,
  pub construction_cost:    f32,
  pub sold_buildings:       f32,
  pub field_purchase:       f32,
  pub field_selling:        f32,
  pub vehicle_running_cost: f32,
  pub vehicle_leasing_cost: f32,
  pub property_maintenance: f32,
  pub property_income:      f32,
  pub production_costs:     f32,
  pub sold_wood:            f32,
  pub sold_bales:           f32,
  pub sold_wool:            f32,
  pub sold_milk:            f32,
  pub sold_products:        f32,
  pub purchase_fuel:        f32,
  pub purchase_seeds:       f32,
  pub purchase_fertilizer:  f32,
  pub purchase_saplings:    f32,
  pub purchase_water:       f32,
  pub purchase_bales:       f32,
  pub purchase_pallets:     f32,
  pub harvest_income:       f32,
  pub income_bga:           f32,
  pub mission_income:       f32,
  pub wage_payment:         f32,
  pub other:                f32,
  pub loan_interest:        f32
}
