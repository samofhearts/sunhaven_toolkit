#[macro_use]
extern crate rocket;
extern crate tera;
extern crate serde;
extern crate serde_json;

// This is like Python Imports
use rocket::serde::Serialize;
use rocket::response::Redirect;
use rocket::form::{Form, Contextual, FromForm, Context};
use rocket::fs::{FileServer, relative};
use csv::Writer;
use csv::ReaderBuilder;
use csv::StringRecord;
use std::fs;
use std::fs::OpenOptions;
use rocket_dyn_templates::Template;


// Check to see if a checkbox is checked

fn checkbox_match(item: &str, form_input: &Context<>) -> String {
    let results = match form_input.field_value(item) {
        Some("true") => item,
        None => "None",
        _ => "None",
    };
    results.to_string()
}



// TrackerSubmission and TrackerSubmit used to create / load tracker files the first time.

#[derive(FromForm)]
//Rust things this struct isn't being used so we add allow dead code so it allows it to be used
//I don't know why it doesn't think it's being used. It's very blatantly being used.
// Validation https://api.rocket.rs/master/rocket/form/validate/index.html
// https://api.rocket.rs/master/rocket/form/validate/fn.contains.html
#[allow(dead_code)]
struct TrackerSubmission<'v> {
    #[field(validate = len(1..))]
    name: &'v str,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct TrackerSubmit<'v> {
    tracker: TrackerSubmission<'v>,
}




// Tons of structs here to match for checkbox check

#[derive(FromForm)]
#[allow(dead_code)]
struct MoneyMuseum<> {
    money_museum_coins: bool,
    money_museum_manaorbs: bool,
    money_museum_tickets: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct BarMuseum<> {
    bar_museum_copperbar: bool,
    bar_museum_ironbar: bool,
    bar_museum_goldbar: bool,
    bar_museum_adamantbar: bool,
    bar_museum_mithrilbar: bool,
    bar_museum_sunitebar: bool,
    bar_museum_elvensteelbar: bool,
    bar_museum_gloritebar: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct GemMuseum<> {
    gem_museum_sapphire: bool,
    gem_museum_ruby: bool,
    gem_museum_amethyst: bool,
    gem_museum_diamond: bool,
    gem_museum_havenite: bool,
    gem_museum_dizzite: bool,
    gem_museum_blackdiamond: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct NelvariMinesMuseum<> {
    nelvari_mines_museum_manashard: bool,
    nelvari_mines_museum_sparklingdragonscale: bool,
    nelvari_mines_museum_sharpdragonscale: bool,
    nelvari_mines_museum_toughdragonscale: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct WithergateMinesMuseum<> {
    withergate_mines_museum_candycorn: bool,
    withergate_mines_museum_rockcandygem: bool,
    withergate_mines_museum_jawbreakergem: bool,
    withergate_mines_museum_hardbutterscotchgem: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct GoldenMuseum<> {
    golden_museum_goldenmilk: bool,
    golden_museum_goldenegg: bool,
    golden_museum_goldenwool: bool,
    golden_museum_goldenlog: bool,
    golden_museum_goldenfeather: bool,
    golden_museum_goldensilk: bool,
    golden_museum_goldenorange: bool,
    golden_museum_goldenstrawberry: bool,
    golden_museum_goldenblueberry: bool,
    golden_museum_goldenpomegranate: bool,
    golden_museum_goldenapple: bool,
    golden_museum_goldenpeach: bool,
    golden_museum_goldenraspberry: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct ManaMuseum<> {
    mana_museum_manadrop: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct ForagingMuseum<> {
    foraging_museum_log: bool,
    foraging_museum_apple: bool,
    foraging_museum_seaweed: bool,
    foraging_museum_blueberry: bool,
    foraging_museum_mushroom: bool,
    foraging_museum_orange: bool,
    foraging_museum_strawberry: bool,
    foraging_museum_berry: bool,
    foraging_museum_raspberry: bool,
    foraging_museum_peach: bool,
    foraging_museum_sanddollar: bool,
    foraging_museum_starfish: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct SpringCropMuseum<> {
    spring_crop_museum_grape: bool,
    spring_crop_museum_wheat: bool,
    spring_crop_museum_tomato: bool,
    spring_crop_museum_corn: bool,
    spring_crop_museum_onion: bool,
    spring_crop_museum_potato: bool,
    spring_crop_museum_greenroot: bool,
    spring_crop_museum_carrot: bool,
    spring_crop_museum_kale: bool,
    spring_crop_museum_lettuce: bool,
    spring_crop_museum_cinnaberry: bool,
    spring_crop_museum_pepper: bool,
    spring_crop_museum_shimmeroot: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct SummerCropMuseum<> {
    summer_crop_museum_armoranth: bool,
    summer_crop_museum_beet: bool,
    summer_crop_museum_lemon: bool,
    summer_crop_museum_chocoberry: bool,
    summer_crop_museum_pineapple: bool,
    summer_crop_museum_pepper: bool,
    summer_crop_museum_melon: bool,
    summer_crop_museum_stormelon: bool,
    summer_crop_museum_durian: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FallCropMuseum<> {
    fall_crop_museum_onion: bool,
    fall_crop_museum_yam: bool,
    fall_crop_museum_sodapopcrop: bool,
    fall_crop_museum_fizzyfruit: bool,
    fall_crop_museum_cranberry: bool,
    fall_crop_museum_barley: bool,
    fall_crop_museum_pumpkin: bool,
    fall_crop_museum_ghostpepper: bool,
    fall_crop_museum_butternut: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct WinterCropMuseum<> {
    winter_crop_museum_tealeaves: bool,
    winter_crop_museum_turnip: bool,
    winter_crop_museum_purpleeggplant: bool,
    winter_crop_museum_heatfruit: bool,
    winter_crop_museum_marshmallowbean: bool,
    winter_crop_museum_brrnana: bool,
    winter_crop_museum_starfruit: bool,
    winter_crop_museum_hexagonberry: bool,
    winter_crop_museum_snowpea: bool,
    winter_crop_museum_snowball: bool,
    winter_crop_museum_blizzardberry: bool,
    winter_crop_museum_balloonfruit: bool,
    winter_crop_museum_pythagoreanberry: bool,
    winter_crop_museum_bluemoonfruit: bool,
    winter_crop_museum_candycane: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FlowersMuseum<> {
    flowers_museum_honeyflower: bool,
    flowers_museum_redrose: bool,
    flowers_museum_bluerose: bool,
    flowers_museum_daisy: bool,
    flowers_museum_orchid: bool,
    flowers_museum_tulip: bool,
    flowers_museum_hibiscus: bool,
    flowers_museum_lavender: bool,
    flowers_museum_sunflower: bool,
    flowers_museum_lily: bool,
    flowers_museum_lotus: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct CombatMuseum<> {
    combat_museum_leafietrinket: bool,
    combat_museum_eliteleafietrinket: bool,
    combat_museum_centipillartrinket: bool,
    combat_museum_peppinchgreentrinket: bool,
    combat_museum_scorpeppertrinket: bool,
    combat_museum_elitescorpeppertrinket: bool,
    combat_museum_hatcrabtrinket: bool,
    combat_museum_floatycrabtrinket: bool,
    combat_museum_bucketcrabtrinket: bool,
    combat_museum_umbrellacrabtrinket: bool,
    combat_museum_chimchucktrinket: bool,
    combat_museum_ancientsunhavensword: bool,
    combat_museum_ancientnelvarisword: bool,
    combat_museum_ancientwithergatesword: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct ExplorationMuseum<> {
    exploration_museum_petrifiedlog: bool,
    exploration_museum_phoenixfeather: bool,
    exploration_museum_fairywings: bool,
    exploration_museum_griffonegg: bool,
    exploration_museum_manasap: bool,
    exploration_museum_pumicestone: bool,
    exploration_museum_mysteriousantler: bool,
    exploration_museum_dragonfang: bool,
    exploration_museum_monstercandy: bool,
    exploration_museum_unicornhairtuft: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct AlchemyMuseum<> {
    alchemy_museum_manapotion: bool,
    alchemy_museum_healthpotion: bool,
    alchemy_museum_attackpotion: bool,
    alchemy_museum_speedpotion: bool,
    alchemy_museum_defensepotion: bool,
    alchemy_museum_advancedattackpotion: bool,
    alchemy_museum_advanceddefensepotion: bool,
    alchemy_museum_advancedspellpotion: bool,
    alchemy_museum_incredibleattackpotion: bool,
    alchemy_museum_incredibledefensepotion: bool,
    alchemy_museum_incrediblespellpotion: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct NelvariFarmingMuseum<> {
    nelvari_farming_museum_acorn: bool,
    nelvari_farming_museum_rockfruit: bool,
    nelvari_farming_museum_waterfruit: bool,
    nelvari_farming_museum_firefruit: bool,
    nelvari_farming_museum_walkchoy: bool,
    nelvari_farming_museum_windchime: bool,
    nelvari_farming_museum_shiiwalkimushroom: bool,
    nelvari_farming_museum_dragonfruit: bool,
    nelvari_farming_museum_managem: bool,
    nelvari_farming_museum_cattail: bool,
    nelvari_farming_museum_indiglow: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct NelvariTempleMuseum<> {
    nelvari_temple_museum_originsofthegrandtreeandnivarapt1: bool,
    nelvari_temple_museum_originsofthegrandtreeandnivarapt2: bool,
    nelvari_temple_museum_originsofthegrandtreeandnivarapt3: bool,
    nelvari_temple_museum_originsofthegrandtreeandnivarapt4: bool,
    nelvari_temple_museum_originsofthegrandtreeandnivarapt5: bool,
    nelvari_temple_museum_originsofsunhavenandeliospt1: bool,
    nelvari_temple_museum_originsofsunhavenandeliospt2: bool,
    nelvari_temple_museum_originsofsunhavenandeliospt3: bool,
    nelvari_temple_museum_originsofsunhavenandeliospt4: bool,
    nelvari_temple_museum_originsofsunhavenandeliospt5: bool,
    nelvari_temple_museum_originsofdynusandshadowspt1: bool,
    nelvari_temple_museum_originsofdynusandshadowspt2: bool,
    nelvari_temple_museum_originsofdynusandshadowspt3: bool,
    nelvari_temple_museum_originsofdynusandshadowspt4: bool,
    nelvari_temple_museum_originsofdynusandshadowspt5: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct WithergateFarmingMuseum<> {
    withergate_farming_museum_krakenkale: bool,
    withergate_farming_museum_tombmelon: bool,
    withergate_farming_museum_suckerstem: bool,
    withergate_farming_museum_razorstalk: bool,
    withergate_farming_museum_snappyplant: bool,
    withergate_farming_museum_moonplant: bool,
    withergate_farming_museum_eggplant: bool,
    withergate_farming_museum_demonorb: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FishingRelicMuseum<> {
    fishing_relic_museum_handmadebobber: bool,
    fishing_relic_museum_ancientmagicstaff: bool,
    fishing_relic_museum_bronzedragonrelic: bool,
    fishing_relic_museum_oldswordhilt: bool,
    fishing_relic_museum_nelvarirunestone: bool,
    fishing_relic_museum_ancientelvenheaddress: bool,
    fishing_relic_museum_oldmayoralpainting: bool,
    fishing_relic_museum_tentaclemonsteremblem: bool,
    fishing_relic_museum_ancientangelquill: bool,
    fishing_relic_museum_ancientnagacrook: bool,
    fishing_relic_museum_ancientamaritotem: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct SpringFishMuseum<> {
    spring_fish_museum_butterflyfish: bool,
    spring_fish_museum_sunfish: bool,
    spring_fish_museum_flowerflounder: bool,
    spring_fish_museum_raincloudray: bool,
    spring_fish_museum_floraltrout: bool,
    spring_fish_museum_neontetra: bool,
    spring_fish_museum_seahorse: bool,
    spring_fish_museum_paintedegg: bool,
    spring_fish_museum_tadpole: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct SummerFishMuseum<> {
    summer_fish_museum_blazeel: bool,
    summer_fish_museum_hearthangler: bool,
    summer_fish_museum_scorchingsquid: bool,
    summer_fish_museum_magmastar: bool,
    summer_fish_museum_tinderturtle: bool,
    summer_fish_museum_pyrelus: bool,
    summer_fish_museum_flameray: bool,
    summer_fish_museum_moltenslug: bool,
    summer_fish_museum_searback: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FallFishMuseum<> {
    fall_fish_museum_coducopia: bool,
    fall_fish_museum_kingsalmon: bool,
    fall_fish_museum_hayfish: bool,
    fall_fish_museum_acornanchovy: bool,
    fall_fish_museum_vampirepiranha: bool,
    fall_fish_museum_ghostfish: bool,
    fall_fish_museum_pumpkinjelly: bool,
    fall_fish_museum_pirateperch: bool,
    fall_fish_museum_autumnleafsole: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct WinterFishMuseum<> {
    winter_fish_museum_frostfin: bool,
    winter_fish_museum_christmaslightfish: bool,
    winter_fish_museum_hollycarp: bool,
    winter_fish_museum_jinglebass: bool,
    winter_fish_museum_frozentuna: bool,
    winter_fish_museum_scarffish: bool,
    winter_fish_museum_heatfin: bool,
    winter_fish_museum_iciclecarp: bool,
    winter_fish_museum_blazingherring: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct NelvariFishMuseum<> {
    nelvari_fish_museum_robedparrotfish: bool,
    nelvari_fish_museum_axolotl: bool,
    nelvari_fish_museum_frilledbetta: bool,
    nelvari_fish_museum_horsefish: bool,
    nelvari_fish_museum_flamefish: bool,
    nelvari_fish_museum_dragongulper: bool,
    nelvari_fish_museum_neapolitanfish: bool,
    nelvari_fish_museum_snobfish: bool,
    nelvari_fish_museum_kelpeel: bool,
    nelvari_fish_museum_princelyfrog: bool,
    nelvari_fish_museum_angelfin: bool,
    nelvari_fish_museum_bubblefish: bool,
    nelvari_fish_museum_crystaltetra: bool,
    nelvari_fish_museum_skyray: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct WithergateFishMuseum<> {
    withergate_fish_museum_kraken: bool,
    withergate_fish_museum_waterbear: bool,
    withergate_fish_museum_bonemouthbass: bool,
    withergate_fish_museum_mummytrout: bool,
    withergate_fish_museum_deadeyeshrimp: bool,
    withergate_fish_museum_electriceel: bool,
    withergate_fish_museum_brainjelly: bool,
    withergate_fish_museum_redfinnedpincher: bool,
    withergate_fish_museum_seabat: bool,
    withergate_fish_museum_ghostheadtuna: bool,
    withergate_fish_museum_globfish: bool,
    withergate_fish_museum_livingjelly: bool,
    withergate_fish_museum_purrmaid: bool,
    withergate_fish_museum_slimeleech: bool,
    withergate_fish_museum_goblinshark: bool,
    withergate_fish_museum_moonfish: bool,
    withergate_fish_museum_toothyangler: bool,
    withergate_fish_museum_vampiresquid: bool,
    withergate_fish_museum_viperfish: bool,
    withergate_fish_museum_albinosquid: bool,
    withergate_fish_museum_devilfin: bool,
    withergate_fish_museum_shadowtuna: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct LargeFishTankMuseum<> {
    large_fish_tank_museum_pygmytuna: bool,
    large_fish_tank_museum_catfish: bool,
    large_fish_tank_museum_goldfish: bool,
    large_fish_tank_museum_streamlinecod: bool,
    large_fish_tank_museum_salmon: bool,
    large_fish_tank_museum_clownfish: bool,
    large_fish_tank_museum_blackbass: bool,
    large_fish_tank_museum_rainbowtrout: bool,
    large_fish_tank_museum_popeyegoldfish: bool,
    large_fish_tank_museum_pufferfish: bool,
    large_fish_tank_museum_ironheadsturgeon: bool,
    large_fish_tank_museum_cuddlefish: bool,
    large_fish_tank_museum_lobster: bool,
    large_fish_tank_museum_silvercarp: bool,
    large_fish_tank_museum_tuna: bool,
    large_fish_tank_museum_bluntedswordfish: bool,
    large_fish_tank_museum_ribboneel: bool,
    large_fish_tank_museum_tigertrout: bool,
    large_fish_tank_museum_eel: bool,
    large_fish_tank_museum_redsnapper: bool,
    large_fish_tank_museum_carp: bool,
    large_fish_tank_museum_redeyepiranha: bool,
    large_fish_tank_museum_angelfish: bool,
    large_fish_tank_museum_whitebellyshark: bool,
    large_fish_tank_museum_koifish: bool,
    large_fish_tank_museum_sandstonefish: bool,
}


#[derive(FromForm)]
#[allow(dead_code)]
struct RareAltar<> {
    rare_altar_adventurekeepsake: bool,
    rare_altar_richeskeepsake: bool,
    rare_altar_romancekeepsake: bool,
    rare_altar_peacekeepsake: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FruitAltar<> {
    fruit_altar_raspberries: bool,
    fruit_altar_peach: bool,
    fruit_altar_orange: bool,
    fruit_altar_blueberries: bool,
    fruit_altar_berry: bool,
    fruit_altar_apple: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct ForagingAltar<> {
    foraging_altar_log: bool,
    foraging_altar_firecrystal: bool,
    foraging_altar_earthcrystal: bool,
    foraging_altar_watercrystal: bool,
    foraging_altar_sanddollar: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct GoldAltar<> {
    gold_altar_coins: bool,
    gold_altar_goldbar: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct CookingAltar<> {
    cooking_altar_cheesecake: bool,
    cooking_altar_spicyramen: bool,
    cooking_altar_sesamericeball: bool,
    cooking_altar_pizza: bool,
    cooking_altar_cookies: bool,
    cooking_altar_coffee: bool,
    cooking_altar_tomatosoup: bool,
    cooking_altar_shimmerroottreat: bool,
    cooking_altar_energysmoothie: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct MiningAltar<> {
    mining_altar_stone: bool,
    mining_altar_coal: bool,
    mining_altar_copperore: bool,
    mining_altar_sapphire: bool,
    mining_altar_ruby: bool,
    mining_altar_amethyst: bool,
    mining_altar_diamond: bool,
    mining_altar_havenite: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FarmingAltar<> {
    farming_altar_wheat: bool,
    farming_altar_corn: bool,
    farming_altar_potatoes: bool,
    farming_altar_tomato: bool,
    farming_altar_carrot: bool,
    farming_altar_sugarcane: bool,
    farming_altar_onion: bool,
    farming_altar_greenroot: bool,
    farming_altar_honeyflower: bool,
    farming_altar_rice: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct FishAltar<> {
    fish_altar_dorado: bool,
    fish_altar_duorado: bool,
    fish_altar_crab: bool,
    fish_altar_seabass: bool,
    fish_altar_goldfish: bool,
    fish_altar_bonemouthbass: bool,
    fish_altar_chromafin: bool,
    fish_altar_goldencarp: bool,
    fish_altar_flamefish: bool,
    fish_altar_purrmaid: bool,
    fish_altar_crystaltetra: bool,
    fish_altar_skyray: bool,
}

#[derive(FromForm)]
#[allow(dead_code)]
struct TicketsAltar<> {
    tickets_altar_tickets: bool,
}







#[derive(FromForm)]
#[allow(dead_code)]
struct MenuSave<'v> {
    money_museum: MoneyMuseum<>,
    bar_museum: BarMuseum<>,
    gem_museum: GemMuseum<>,
    nelvari_mines_museum: NelvariMinesMuseum<>,
    withergate_mines_museum: WithergateMinesMuseum<>,
    golden_museum: GoldenMuseum<>,
    mana_museum: ManaMuseum<>,
    foraging_museum: ForagingMuseum<>,
    spring_crop_museum: SpringCropMuseum<>,
    summer_crop_museum: SummerCropMuseum<>,
    fall_crop_museum: FallCropMuseum<>,
    winter_crop_museum: WinterCropMuseum<>,
    flowers_museum: FlowersMuseum<>,
    combat_museum: CombatMuseum<>,
    exploration_museum: ExplorationMuseum<>,
    alchemy_museum: AlchemyMuseum<>,
    nelvari_farming_museum: NelvariFarmingMuseum<>,
    nelvari_temple_museum: NelvariTempleMuseum<>,
    withergate_farming_museum: WithergateFarmingMuseum<>,
    fishing_relic_museum: FishingRelicMuseum<>,
    spring_fish_museum: SpringFishMuseum<>,
    summer_fish_museum: SummerFishMuseum<>,
    fall_fish_museum: FallFishMuseum<>,
    winter_fish_museum: WinterFishMuseum<>,
    nelvari_fish_museum: NelvariFishMuseum<>,
    withergate_fish_museum: WithergateFishMuseum<>,
    large_fish_tank_museum: LargeFishTankMuseum<>,
    rare_altar: RareAltar<>,
    fruit_altar: FruitAltar<>,
    foraging_altar: ForagingAltar<>,
    gold_altar: GoldAltar<>,
    cooking_altar: CookingAltar<>,
    mining_altar: MiningAltar<>,
    farming_altar: FarmingAltar<>,
    fish_altar: FishAltar<>,
    tickets_altar: TicketsAltar<>,
    tracker: TrackerSubmission<'v>,
}






// If somebody browses to base URL load index
#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[get("/")]
fn new() -> Template {
    Template::render("new", &Context::default())
}


#[derive(Serialize)]
struct LoadContext<'a>{
    files: Vec<String>,
    errors: &'a str,
    values: &'a str,
}

#[get("/")]
fn load() -> Template {
    // Vector we're going to fill with available file options
    let mut files: Vec<String> = Vec::new();

    // Path Rust should look for available csv files
    let save_path = fs::read_dir("./csv/").unwrap();

    // Loop and only take out actual .csv files
    for path in save_path {
        let file = path.unwrap().file_name().into_string().unwrap();
        if file.contains(".csv") {
            // Add files to the Vector
            files.push(file.replace(".csv",""));
        }
    }

    Template::render("load", &LoadContext{
        files: files,
        errors: "",
        values: "values",
    })
}






// Main community / altar tracker page

#[derive(Serialize)]
struct TrackerContext<'a>{
    tracker: &'a str,
    errors: &'a str,
    values: &'a str,
}


#[post("/<tracker>")]
fn menu(tracker: &str) -> Template {

    let mut values_str = String::from("");

    let mut file_reader = ReaderBuilder::new()
                        .has_headers(false)
                        .from_path(format!("./csv/{}.csv",tracker)).unwrap();
    let read_file = file_reader.records().collect::<Result<Vec<StringRecord>, csv::Error>>().unwrap();

    for row in read_file.iter() {
        for entry in row.iter() {
            values_str = values_str + "," + entry;
        }
    }        

    Template::render("menu", &TrackerContext{
        tracker: tracker,
        errors: "",
        values: values_str.as_str(),
    })
}



//
// SAVE DATA TO CSV FOR FUTURE LOADING
//
#[post("/", data = "<form>")]
fn save<'r>(form: Form<Contextual<'r, MenuSave<>>>) -> Redirect {
    let redirect = match form.value {
        Some(_) => {
            let tracker_name: &str = &form.context.field_value("tracker.name").expect("Name should not be empty");
            // Write the contents of the form fields to a CSV
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .append(false)
                .open(format!("csv/{}.csv", tracker_name))
                .unwrap();
            let mut wtr = Writer::from_writer(file);

            // There has to be a better way to do this, but I haven't figured it out yet.
            // Probably could load all this with a loop and an external file containing all the data to fill the Vecs
            let mut money_museum_vec: Vec<&str> = vec!["money_museum_coins","money_museum_manaorbs","money_museum_tickets"];
            let mut bar_museum_vec: Vec<&str> = vec!["bar_museum_copperbar","bar_museum_ironbar","bar_museum_goldbar","bar_museum_adamantbar","bar_museum_mithrilbar","bar_museum_sunitebar","bar_museum_elvensteelbar","bar_museum_gloritebar"];
            let mut gem_museum_vec: Vec<&str> = vec!["gem_museum_sapphire","gem_museum_ruby","gem_museum_amethyst","gem_museum_diamond","gem_museum_havenite","gem_museum_dizzite","gem_museum_blackdiamond"];
            let mut nelvari_mines_museum_vec: Vec<&str> = vec!["nelvari_mines_museum_manashard","nelvari_mines_museum_sparklingdragonscale","nelvari_mines_museum_sharpdragonscale","nelvari_mines_museum_toughdragonscale"];
            let mut withergate_mines_museum_vec: Vec<&str> = vec!["withergate_mines_museum_candycorn","withergate_mines_museum_rockcandygem","withergate_mines_museum_jawbreakergem","withergate_mines_museum_hardbutterscotchgem"];
            let mut golden_museum_vec: Vec<&str> = vec!["golden_museum_goldenmilk","golden_museum_goldenegg","golden_museum_goldenwool","golden_museum_goldenlog","golden_museum_goldenfeather","golden_museum_goldensilk","golden_museum_goldenorange","golden_museum_goldenstrawberry","golden_museum_goldenblueberry","golden_museum_goldenpomegranate","golden_museum_goldenapple","golden_museum_goldenpeach","golden_museum_goldenraspberry"];
            let mut mana_museum_vec: Vec<&str> = vec!["mana_museum_manadrop"];
            let mut foraging_museum_vec: Vec<&str> = vec!["foraging_museum_log","foraging_museum_apple","foraging_museum_seaweed","foraging_museum_blueberry","foraging_museum_mushroom","foraging_museum_orange","foraging_museum_strawberry","foraging_museum_berry","foraging_museum_raspberry","foraging_museum_peach","foraging_museum_sanddollar","foraging_museum_starfish"];
            let mut spring_crop_museum_vec: Vec<&str> = vec!["spring_crop_museum_grape","spring_crop_museum_wheat","spring_crop_museum_tomato","spring_crop_museum_corn","spring_crop_museum_onion","spring_crop_museum_potato","spring_crop_museum_greenroot","spring_crop_museum_carrot","spring_crop_museum_kale","spring_crop_museum_lettuce","spring_crop_museum_cinnaberry","spring_crop_museum_pepper","spring_crop_museum_shimmeroot"];
            let mut summer_crop_museum_vec: Vec<&str> = vec!["summer_crop_museum_armoranth","summer_crop_museum_beet","summer_crop_museum_lemon","summer_crop_museum_chocoberry","summer_crop_museum_pineapple","summer_crop_museum_pepper","summer_crop_museum_melon","summer_crop_museum_stormelon","summer_crop_museum_durian"];
            let mut fall_crop_museum_vec: Vec<&str> = vec!["fall_crop_museum_onion","fall_crop_museum_yam","fall_crop_museum_sodapopcrop","fall_crop_museum_fizzyfruit","fall_crop_museum_cranberry","fall_crop_museum_barley","fall_crop_museum_pumpkin","fall_crop_museum_ghostpepper","fall_crop_museum_butternut"];
            let mut winter_crop_museum_vec: Vec<&str> = vec!["winter_crop_museum_tealeaves","winter_crop_museum_turnip","winter_crop_museum_purpleeggplant","winter_crop_museum_heatfruit","winter_crop_museum_marshmallowbean","winter_crop_museum_brrnana","winter_crop_museum_starfruit","winter_crop_museum_hexagonberry","winter_crop_museum_snowpea","winter_crop_museum_snowball","winter_crop_museum_blizzardberry","winter_crop_museum_balloonfruit","winter_crop_museum_pythagoreanberry","winter_crop_museum_bluemoonfruit","winter_crop_museum_candycane"];
            let mut flowers_museum_vec: Vec<&str> = vec!["flowers_museum_honeyflower","flowers_museum_redrose","flowers_museum_bluerose","flowers_museum_daisy","flowers_museum_orchid","flowers_museum_tulip","flowers_museum_hibiscus","flowers_museum_lavender","flowers_museum_sunflower","flowers_museum_lily","flowers_museum_lotus"];
            let mut combat_museum_vec: Vec<&str> = vec!["combat_museum_leafietrinket","combat_museum_eliteleafietrinket","combat_museum_centipillartrinket","combat_museum_peppinchgreentrinket","combat_museum_scorpeppertrinket","combat_museum_elitescorpeppertrinket","combat_museum_hatcrabtrinket","combat_museum_floatycrabtrinket","combat_museum_bucketcrabtrinket","combat_museum_umbrellacrabtrinket","combat_museum_chimchucktrinket","combat_museum_ancientsunhavensword","combat_museum_ancientnelvarisword","combat_museum_ancientwithergatesword"];
            let mut exploration_museum_vec: Vec<&str> = vec!["exploration_museum_petrifiedlog","exploration_museum_phoenixfeather","exploration_museum_fairywings","exploration_museum_griffonegg","exploration_museum_manasap","exploration_museum_pumicestone","exploration_museum_mysteriousantler","exploration_museum_dragonfang","exploration_museum_monstercandy","exploration_museum_unicornhairtuft"];
            let mut alchemy_museum_vec: Vec<&str> = vec!["alchemy_museum_manapotion","alchemy_museum_healthpotion","alchemy_museum_attackpotion","alchemy_museum_speedpotion","alchemy_museum_defensepotion","alchemy_museum_advancedattackpotion","alchemy_museum_advanceddefensepotion","alchemy_museum_advancedspellpotion","alchemy_museum_incredibleattackpotion","alchemy_museum_incredibledefensepotion","alchemy_museum_incrediblespellpotion"];
            let mut nelvari_farming_museum_vec: Vec<&str> = vec!["nelvari_farming_museum_acorn","nelvari_farming_museum_rockfruit","nelvari_farming_museum_waterfruit","nelvari_farming_museum_firefruit","nelvari_farming_museum_walkchoy","nelvari_farming_museum_windchime","nelvari_farming_museum_shiiwalkimushroom","nelvari_farming_museum_dragonfruit","nelvari_farming_museum_managem","nelvari_farming_museum_cattail","nelvari_farming_museum_indiglow"];
            let mut nelvari_temple_museum_vec: Vec<&str> = vec!["nelvari_temple_museum_originsofthegrandtreeandnivarapt1","nelvari_temple_museum_originsofthegrandtreeandnivarapt2","nelvari_temple_museum_originsofthegrandtreeandnivarapt3","nelvari_temple_museum_originsofthegrandtreeandnivarapt4","nelvari_temple_museum_originsofthegrandtreeandnivarapt5","nelvari_temple_museum_originsofsunhavenandeliospt1","nelvari_temple_museum_originsofsunhavenandeliospt2","nelvari_temple_museum_originsofsunhavenandeliospt3","nelvari_temple_museum_originsofsunhavenandeliospt4","nelvari_temple_museum_originsofsunhavenandeliospt5","nelvari_temple_museum_originsofdynusandshadowspt1","nelvari_temple_museum_originsofdynusandshadowspt2","nelvari_temple_museum_originsofdynusandshadowspt3","nelvari_temple_museum_originsofdynusandshadowspt4","nelvari_temple_museum_originsofdynusandshadowspt5"];
            let mut withergate_farming_museum_vec: Vec<&str> = vec!["withergate_farming_museum_krakenkale","withergate_farming_museum_tombmelon","withergate_farming_museum_suckerstem","withergate_farming_museum_razorstalk","withergate_farming_museum_snappyplant","withergate_farming_museum_moonplant","withergate_farming_museum_eggplant","withergate_farming_museum_demonorb"];
            let mut fishing_relic_museum_vec: Vec<&str> = vec!["fishing_relic_museum_handmadebobber","fishing_relic_museum_ancientmagicstaff","fishing_relic_museum_bronzedragonrelic","fishing_relic_museum_oldswordhilt","fishing_relic_museum_nelvarirunestone","fishing_relic_museum_ancientelvenheaddress","fishing_relic_museum_oldmayoralpainting","fishing_relic_museum_tentaclemonsteremblem","fishing_relic_museum_ancientangelquill","fishing_relic_museum_ancientnagacrook","fishing_relic_museum_ancientamaritotem"];
            let mut spring_fish_museum_vec: Vec<&str> = vec!["spring_fish_museum_butterflyfish","spring_fish_museum_sunfish","spring_fish_museum_flowerflounder","spring_fish_museum_raincloudray","spring_fish_museum_floraltrout","spring_fish_museum_neontetra","spring_fish_museum_seahorse","spring_fish_museum_paintedegg","spring_fish_museum_tadpole"];
            let mut summer_fish_museum_vec: Vec<&str> = vec!["summer_fish_museum_blazeel","summer_fish_museum_hearthangler","summer_fish_museum_scorchingsquid","summer_fish_museum_magmastar","summer_fish_museum_tinderturtle","summer_fish_museum_pyrelus","summer_fish_museum_flameray","summer_fish_museum_moltenslug","summer_fish_museum_searback"];
            let mut fall_fish_museum_vec: Vec<&str> = vec!["fall_fish_museum_coducopia","fall_fish_museum_kingsalmon","fall_fish_museum_hayfish","fall_fish_museum_acornanchovy","fall_fish_museum_vampirepiranha","fall_fish_museum_ghostfish","fall_fish_museum_pumpkinjelly","fall_fish_museum_pirateperch","fall_fish_museum_autumnleafsole"];
            let mut winter_fish_museum_vec: Vec<&str> = vec!["winter_fish_museum_frostfin","winter_fish_museum_christmaslightfish","winter_fish_museum_hollycarp","winter_fish_museum_jinglebass","winter_fish_museum_frozentuna","winter_fish_museum_scarffish","winter_fish_museum_heatfin","winter_fish_museum_iciclecarp","winter_fish_museum_blazingherring"];
            let mut nelvari_fish_museum_vec: Vec<&str> = vec!["nelvari_fish_museum_robedparrotfish","nelvari_fish_museum_axolotl","nelvari_fish_museum_frilledbetta","nelvari_fish_museum_horsefish","nelvari_fish_museum_flamefish","nelvari_fish_museum_dragongulper","nelvari_fish_museum_neapolitanfish","nelvari_fish_museum_snobfish","nelvari_fish_museum_kelpeel","nelvari_fish_museum_princelyfrog","nelvari_fish_museum_angelfin","nelvari_fish_museum_bubblefish","nelvari_fish_museum_crystaltetra","nelvari_fish_museum_skyray"];
            let mut withergate_fish_museum_vec: Vec<&str> = vec!["withergate_fish_museum_kraken","withergate_fish_museum_waterbear","withergate_fish_museum_bonemouthbass","withergate_fish_museum_mummytrout","withergate_fish_museum_deadeyeshrimp","withergate_fish_museum_electriceel","withergate_fish_museum_brainjelly","withergate_fish_museum_redfinnedpincher","withergate_fish_museum_seabat","withergate_fish_museum_ghostheadtuna","withergate_fish_museum_globfish","withergate_fish_museum_livingjelly","withergate_fish_museum_purrmaid","withergate_fish_museum_slimeleech","withergate_fish_museum_goblinshark","withergate_fish_museum_moonfish","withergate_fish_museum_toothyangler","withergate_fish_museum_vampiresquid","withergate_fish_museum_viperfish","withergate_fish_museum_albinosquid","withergate_fish_museum_devilfin","withergate_fish_museum_shadowtuna"];
            let mut large_fish_tank_museum_vec: Vec<&str> = vec!["large_fish_tank_museum_pygmytuna","large_fish_tank_museum_catfish","large_fish_tank_museum_goldfish","large_fish_tank_museum_streamlinecod","large_fish_tank_museum_salmon","large_fish_tank_museum_clownfish","large_fish_tank_museum_blackbass","large_fish_tank_museum_rainbowtrout","large_fish_tank_museum_popeyegoldfish","large_fish_tank_museum_pufferfish","large_fish_tank_museum_ironheadsturgeon","large_fish_tank_museum_cuddlefish","large_fish_tank_museum_lobster","large_fish_tank_museum_silvercarp","large_fish_tank_museum_tuna","large_fish_tank_museum_bluntedswordfish","large_fish_tank_museum_ribboneel","large_fish_tank_museum_tigertrout","large_fish_tank_museum_eel","large_fish_tank_museum_redsnapper","large_fish_tank_museum_carp","large_fish_tank_museum_redeyepiranha","large_fish_tank_museum_angelfish","large_fish_tank_museum_whitebellyshark","large_fish_tank_museum_koifish","large_fish_tank_museum_sandstonefish"];
            let mut rare_altar_vec: Vec<&str> = vec!["rare_altar_adventurekeepsake","rare_altar_richeskeepsake","rare_altar_romancekeepsake","rare_altar_peacekeepsake"];
            let mut fruit_altar_vec: Vec<&str> = vec!["fruit_altar_raspberries","fruit_altar_peach","fruit_altar_orange","fruit_altar_blueberries","fruit_altar_berry","fruit_altar_apple"];
            let mut foraging_altar_vec: Vec<&str> = vec!["foraging_altar_log","foraging_altar_firecrystal","foraging_altar_earthcrystal","foraging_altar_watercrystal","foraging_altar_sanddollar"];
            let mut gold_altar_vec: Vec<&str> = vec!["gold_altar_coins","gold_altar_goldbar"];
            let mut cooking_altar_vec: Vec<&str> = vec!["cooking_altar_cheesecake","cooking_altar_spicyramen","cooking_altar_sesamericeball","cooking_altar_pizza","cooking_altar_cookies","cooking_altar_coffee","cooking_altar_tomatosoup","cooking_altar_shimmerroottreat","cooking_altar_energysmoothie"];
            let mut mining_altar_vec: Vec<&str> = vec!["mining_altar_stone","mining_altar_coal","mining_altar_copperore","mining_altar_sapphire","mining_altar_ruby","mining_altar_amethyst","mining_altar_diamond","mining_altar_havenite"];
            let mut farming_altar_vec: Vec<&str> = vec!["farming_altar_wheat","farming_altar_corn","farming_altar_potatoes","farming_altar_tomato","farming_altar_carrot","farming_altar_sugarcane","farming_altar_onion","farming_altar_greenroot","farming_altar_honeyflower","farming_altar_rice"];
            let mut fish_altar_vec:  Vec<&str> = vec!["fish_altar_dorado","fish_altar_duorado","fish_altar_crab","fish_altar_seabass","fish_altar_goldfish","fish_altar_bonemouthbass","fish_altar_chromafin","fish_altar_goldencarp","fish_altar_flamefish","fish_altar_purrmaid","fish_altar_crystaltetra","fish_altar_skyray"];
            let mut tickets_altar_vec: Vec<&str> = vec!["tickets_altar_tickets"];
            let mut all_checked_items: Vec<String> = Vec::new();
            let mut all_items: Vec<&str> = Vec::new();
            
            // Again, definitely better ways to do this, but it works for now.
            all_items.append(&mut money_museum_vec);
            all_items.append(&mut bar_museum_vec);
            all_items.append(&mut gem_museum_vec);
            all_items.append(&mut nelvari_mines_museum_vec);
            all_items.append(&mut withergate_mines_museum_vec);
            all_items.append(&mut golden_museum_vec);
            all_items.append(&mut mana_museum_vec);
            all_items.append(&mut foraging_museum_vec);
            all_items.append(&mut spring_crop_museum_vec);
            all_items.append(&mut summer_crop_museum_vec);
            all_items.append(&mut fall_crop_museum_vec);
            all_items.append(&mut winter_crop_museum_vec);
            all_items.append(&mut flowers_museum_vec);
            all_items.append(&mut combat_museum_vec);
            all_items.append(&mut exploration_museum_vec);
            all_items.append(&mut alchemy_museum_vec);
            all_items.append(&mut nelvari_farming_museum_vec);
            all_items.append(&mut nelvari_temple_museum_vec);
            all_items.append(&mut withergate_farming_museum_vec);
            all_items.append(&mut fishing_relic_museum_vec);
            all_items.append(&mut spring_fish_museum_vec);
            all_items.append(&mut summer_fish_museum_vec);
            all_items.append(&mut fall_fish_museum_vec);
            all_items.append(&mut winter_fish_museum_vec);
            all_items.append(&mut nelvari_fish_museum_vec);
            all_items.append(&mut withergate_fish_museum_vec);
            all_items.append(&mut large_fish_tank_museum_vec);
            all_items.append(&mut rare_altar_vec);
            all_items.append(&mut fruit_altar_vec);
            all_items.append(&mut foraging_altar_vec);
            all_items.append(&mut gold_altar_vec);
            all_items.append(&mut cooking_altar_vec);
            all_items.append(&mut mining_altar_vec);
            all_items.append(&mut farming_altar_vec);
            all_items.append(&mut fish_altar_vec);
            all_items.append(&mut tickets_altar_vec);

            // Populate all_checked_items VEC ONLY with checkboxed items.
            for item in all_items.iter() {
                all_checked_items.push(checkbox_match(item,&form.context));
            }
            
            // actual writing the csv record (row)
            wtr.write_record(all_checked_items).expect("File Write Error, Permissions?");

            // You can't remove this because this function is expecting a return. This IS the return because it doesn't end in a semicolon
            Redirect::temporary(format!("/menu/{}", tracker_name))
        }
        None => Redirect::temporary(uri!(index)),
    };

    redirect
}

//
// REDIRECTS TO MAIN MENU WITH TRACKING NUMBER ADDED
//

#[post("/", data = "<form>")]
fn creating_new<'r>(form: Form<Contextual<'r, TrackerSubmit<'r>>>) -> Redirect {
    let tracker_name: &str = &form.context.field_value("tracker.name").expect("Name should not be empty");

    // Write the contents of the form fields to a CSV
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(format!("csv/{}.csv", tracker_name))
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    
    // actual writing the csv record (row) This just garbage data that will be overwritten the first time the user saves
    wtr.write_record(["test","no","no"]).expect("File Write Error, Permissions?");

    Redirect::temporary(format!("/menu/{}", tracker_name))
}




#[post("/", data = "<form>")]
fn loading<'r>(form: Form<Contextual<'r, TrackerSubmit<'r>>>) -> Redirect {
    let tracker_name: &str = &form.context.field_value("selections.load").expect("Name should not be empty");
    Redirect::temporary(format!("/menu/{}", tracker_name))
}



// This either saves or updates depending on which buttont he user pushes.
// If this "updates" it's really just re-calling the page to re-load the csv again.
// The reason these 2 are put together is so that both of the buttons in menu.html.tera can be in the same form and therefore both be in the sticky header
#[post("/", data = "<form>")]
fn update<'r>(form: Form<Contextual<'r, MenuSave<>>>) -> Redirect {
    let tracker_name: &str = &form.context.field_value("tracker.name").expect("Name should not be empty");
    let submit_value: &str = &form.context.field_value("submit.value").expect("Err");
    if submit_value == "Save" {
        Redirect::temporary(uri!("/menu/save"))
    }
    else {
        Redirect::temporary(format!("/menu/{}", tracker_name))
    }
}


// This actually starts the web server https://rocket.rs/v0.5-rc/guide/getting-started/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/creating_new", routes![creating_new])
        .mount("/loading", routes![loading])
        .mount("/menu/", routes![menu])
        .mount("/menu/save", routes![save])
        .mount("/menu/update", routes![update])
        .mount("/new", routes![new])
        .mount("/load", routes![load])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", FileServer::from(relative!("/static/images")).rank(-1))
}