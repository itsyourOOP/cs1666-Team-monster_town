use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

const STAGE_MULT: f32 = 0.125;
const STAGE_LIMIT: i32 = 6;

pub struct Monster<'a> {
    pub attack_stat: u32,
    pub defense_stat: u32,
    pub moves: Vec<&'a Move>,
    pub monster_type: String,
}

pub struct Move {
    pub name: String,
    pub damage: u32,
    pub self_attack_stages: i32,
    pub self_defense_stages: i32,
    pub opp_attack_stages: i32,
    pub opp_defense_stages: i32,
    pub attack_type: String,
    pub effect: String,
}

pub struct BattleState<'a> {
    pub player_turn: bool,
    pub player_monster: &'a Monster<'a>,
    pub opp_monster: &'a Monster<'a>,
    pub self_attack_stages: i32,
    pub self_defense_stages: i32,
    pub opp_attack_stages: i32,
    pub opp_defense_stages: i32,
}

pub fn load_moves() -> HashMap<String, Move> {
    let reader = BufReader::new(File::open("./data/moves.txt").expect("Cannot open moves.txt"));
    let mut moves = HashMap::new();
    for line in reader.lines().skip(1) {
        let line = line;
        let v = line.unwrap();
        let v = v.split(",").collect::<Vec<&str>>();
        let v: Vec<String> = v.into_iter().map(|d| String::from(d)).collect();
        
        let mov = Move {
            name: v[0].clone(),
            damage: v[1].parse::<u32>().unwrap(),
            self_attack_stages: v[2].parse::<i32>().unwrap(),
            self_defense_stages: v[3].parse::<i32>().unwrap(),
            opp_attack_stages: v[4].parse::<i32>().unwrap(),
            opp_defense_stages: v[5].parse::<i32>().unwrap(),
            attack_type: v[6].clone(),
            effect: v[7].clone(),
        };
        moves.insert(v[0].clone(), mov);
    }
    moves
}
pub fn load_mons<'a>(moves_map: &'a HashMap<String, Move>) -> HashMap<String, Monster<'a>> {
    let reader =
        BufReader::new(File::open("./data/monsters.txt").expect("Cannot open monsters.txt"));
    let mut mons = HashMap::new();
    for line in reader.lines().skip(1) {
        let line = line;
        let v = line.unwrap();
        let v = v.split(",").collect::<Vec<&str>>();
        let v: Vec<String> = v.into_iter().map(|d| String::from(d)).collect();
        let a = &v[4..];
        let moves = a.into_iter().map(|d| &moves_map[d]).collect();

        let mon = Monster {
            attack_stat: v[1].parse::<u32>().unwrap(),
            defense_stat: v[2].parse::<u32>().unwrap(),
            moves: moves,
            monster_type: v[3].clone(),
        };
        mons.insert(v[0].clone(), mon);
    }
    mons
}

pub fn str_effectiveness(damage: f32, attack_type: &String, defense_type: &String) -> Option<String> {
    let a = type_effectiveness(attack_type, defense_type);

    if damage == 0.0 {
        return None;
    }

    return if a == 2.0 {Some(String::from("It was super effective!"))}
    else if a == 0.5 {Some(String::from("It was not very effective."))}
    else {None};
}

fn type_effectiveness(attack_type: &String, defense_type: &String) -> f32 {
    match attack_type.as_str() {
        "Electric" => match defense_type.as_str() {
            "Water" => 2.0,
            _ => 1.0,
        },
        "Flying" => match defense_type.as_str() {
            "Grass" => 2.0,
            _ => 1.0,
        }
        _ => 1.0,
    }
}

fn stab_bonus(type1: &String, type2: &String) -> f32 {
    if type1 == type2 {
        return 2.0;
    };
    return 1.0;
}

fn damage_calc(damage: f32, a: f32, d: f32, stab: f32, typb: f32) -> f32 {
    return (30.0 * damage * (a / d) / 100.0) * stab * typb;
}

pub fn calculate_damage(battle_state: &mut BattleState, move_index: usize) -> f32 {
    if battle_state.player_turn {
        let attack = battle_state.player_monster.moves[move_index];
        calculate_player_attack(
            battle_state,
            attack,
            battle_state.player_monster,
            battle_state.opp_monster,
        )
    } else {
        let attack = battle_state.opp_monster.moves[move_index];
        calculate_opp_attack(
            battle_state,
            attack,
            battle_state.opp_monster,
            battle_state.player_monster,
        )
    }
}

fn calculate_player_attack(
    mut battle_state: &mut BattleState,
    attack: &Move,
    attacker: &Monster,
    opponent: &Monster,
) -> f32 {
    let effective_attack =
        attacker.attack_stat as f32 * (1.0 + STAGE_MULT * battle_state.self_attack_stages as f32);
    let effective_defense =
        opponent.defense_stat as f32 * (1.0 + STAGE_MULT * battle_state.opp_defense_stages as f32);
    let damage = attack.damage as f32;
    let stab_bonus = stab_bonus(&attack.attack_type, &attacker.monster_type);
    let type_bonus = type_effectiveness(&attack.attack_type, &opponent.monster_type);

    let a = damage_calc(
        damage,
        effective_attack,
        effective_defense,
        stab_bonus,
        type_bonus,
    );

    battle_state.self_attack_stages += attack.self_attack_stages;
    battle_state.self_defense_stages += attack.self_defense_stages;
    battle_state.opp_attack_stages += attack.opp_attack_stages;
    battle_state.opp_defense_stages += attack.opp_defense_stages;

    battle_state.opp_attack_stages = battle_state
        .opp_attack_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.opp_defense_stages = battle_state
        .opp_defense_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.self_attack_stages = battle_state
        .self_attack_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.self_defense_stages = battle_state
        .self_defense_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);

    a
}

fn calculate_opp_attack(
    mut battle_state: &mut BattleState,
    attack: &Move,
    attacker: &Monster,
    opponent: &Monster,
) -> f32 {
    let effective_attack =
        attacker.attack_stat as f32 * (1.0 + STAGE_MULT * battle_state.opp_attack_stages as f32);
    let effective_defense =
        opponent.defense_stat as f32 * (1.0 + STAGE_MULT * battle_state.self_defense_stages as f32);
    let damage = attack.damage as f32;
    let stab_bonus = stab_bonus(&attack.attack_type, &attacker.monster_type);
    let type_bonus = type_effectiveness(&attack.attack_type, &opponent.monster_type);

    let a = damage_calc(
        damage,
        effective_attack,
        effective_defense,
        stab_bonus,
        type_bonus,
    );

    battle_state.opp_attack_stages += attack.self_attack_stages;
    battle_state.opp_defense_stages += attack.self_defense_stages;
    battle_state.self_attack_stages += attack.opp_attack_stages;
    battle_state.self_defense_stages += attack.opp_defense_stages;

    battle_state.opp_attack_stages = battle_state
        .opp_attack_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.opp_defense_stages = battle_state
        .opp_defense_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.self_attack_stages = battle_state
        .self_attack_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);
    battle_state.self_defense_stages = battle_state
        .self_defense_stages
        .clamp(-STAGE_LIMIT, STAGE_LIMIT);

    a
}
