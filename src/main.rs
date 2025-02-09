use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;

//
// toml
//
#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,
    xlconfig: xl_config,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    varbool: bool,
    variant_name: String,
    xl_dir_path: String,
    ware_path: String,
    t_path: String,
    out_path: String,
    pageid: String,
    variant_tname: String,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct xl_config {
    trade_purposemod: i32,
    fight_purposemod: i32,
    build_purposemod: i32,
    mine_purposemod: i32,
    auxiliary_purposemod: i32,
    // first order
    mass: Vec<i32>,
    hull: Vec<i32>,
    cargo: Vec<i32>,
    // second order
    hangarcapacity: Vec<i32>,
    unit: Vec<i32>,
    ammo: Vec<i32>,
    // idfk
    explosion: Vec<i32>,

    // movement
    i_pitch: Vec<i32>,
    i_yaw: Vec<i32>,
    i_roll: Vec<i32>,
    forward: Vec<i32>,
    reverse: Vec<i32>,
    horizontal: Vec<i32>,
    vertical: Vec<i32>,
    d_pitch: Vec<i32>,
    d_yaw: Vec<i32>,
    d_roll: Vec<i32>,
}

//
// xml ship
//
#[derive(Deserialize, Debug, Default)]
struct Macros {
    r#macro: NameMacro,
}

#[derive(Deserialize, Debug, Default)]
struct NameMacro {
    name: String,
    class: String,
    component: Component,
    properties: Properties,
}
#[derive(Deserialize, Debug, Default)]
struct Component {
    r#ref: String,
}
#[derive(Deserialize, Debug, Default)]
struct Properties {
    identification: Identification,
    purpose: Purpose,
    hull: Hull,
}
#[derive(Deserialize, Debug, Default)]
struct Identification {
    name: String,
    basename: String,
    description: String,
    variation: String,
    shortvariation: String,
    icon: String,
}
#[derive(Deserialize, Debug, Default)]
struct Purpose {
    primary: String,
    // <purpose primary="fight" />
}
#[derive(Deserialize, Debug, Default)]
struct Hull {
    max: String,
    // <purpose primary="fight" />
}
//
//xml ware
//

#[derive(Deserialize, Debug, Default)]
struct Ware {
    id: String,
    name: String,
    description: String,
    restriction: Restriction,
    owner: Owner,
}
#[derive(Deserialize, Debug, Default)]
struct Restriction {
    licence: String,
}
#[derive(Deserialize, Debug, Default)]
struct Owner {
    faction: String,
}

//
// xml t
//

#[derive(Deserialize, Debug, Default)]
struct t {
    id: String,
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Deserialize, Debug, Default)]
struct Storage {
    id: String,
    #[serde(rename = "$value")]
    content: String,
}
#[derive(Deserialize, Debug, Default)]
struct Shipstorage {
    id: String,
    #[serde(rename = "$value")]
    content: String,
}
/*
<ware id="ship_xen_xl_destroyer_01_a" name="{20101,70501}" description="{20101,70511}" group="ships_xenon" transport="ship" volume="1" tags="noplayerblueprint ship">
    <price min="1033787" average="1216220" max="1398653" />
    <production time="526" amount="1" method="default" name="{20206,601}">
      <primary>
        <ware ware="energycells" amount="2908" />
        <ware ware="ore" amount="2437" />
        <ware ware="silicon" amount="2447" />
      </primary>
    </production>
    <component ref="ship_xen_xl_destroyer_01_a_macro" />
    <restriction licence="capitalship" />
    <owner faction="xenon" />
  </ware>
*/

fn main() {
    // math test

    // math test
    let mut tname = 1;
    let mut tbase = 2;
    let mut tdesc = 3;
    let mut tvar = 4;
    let mut tshort = 5;
    let mut i_string = "".to_string();
    let mut t_string = "".to_string();
    let mut ware_file_string = "".to_string();
    let mut ware_new = "".to_string();
    let toml_str = include_str!("Config.toml");
    let toml_parsed: Toml = toml::from_str(&toml_str).unwrap();
    let variant = &toml_parsed.config.varbool;
    let t_path = &toml_parsed.config.t_path;
    let unwrapped_tfile = fs::read_to_string(t_path).unwrap();

    let mut macro_relations = HashMap::new();
    let mut cargo_vec: Vec<String> = vec![];
    let mut shipstorage_vec: Vec<String> = vec![];

    // invariant!!! - this reads the ships before the storage only because its somehow alphabetized
    for entry in fs::read_dir(&toml_parsed.config.xl_dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            tbase += 100;
            tvar += 100;
            tshort += 100;
            tname += 100;
            tdesc += 100;
            let mut macro_string = fs::read_to_string(&path).unwrap();

            let macro_parsed: Macros = serde_xml_rs::from_str(&macro_string).unwrap_or_default();
            let macroname = &path.file_name().unwrap().to_str().unwrap();

            if toml_parsed.config.varbool == true {
                let pattern = &macro_parsed.r#macro.name;
                if pattern != "" {
                    let namecombo = &macroname
                        .replace(".xml", "")
                        .replace("_macro", &[&toml_parsed.config.variant_name.as_str(), "_macro"].concat());
                    macro_string = replace_pattern(pattern, &macro_string, namecombo)
                    i_add(i_string,namecombo,"")
                }
                let pattern = &macro_parsed.r#macro.properties.identification.name;
                if pattern != "" {
                    //tfile
                    let tname_line = get_tfile_value(&pattern, &unwrapped_tfile);
                    t_string.push_str(&format!("\n{}", &tfile_ware(tname, tname_line, &toml_parsed)));
                    //macro
                    macro_string = replace_pattern(&pattern, &macro_string, &format!("{{{},{}}}", &toml_parsed.config.pageid, tname.to_string()));
                }
                let pattern = &macro_parsed.r#macro.properties.identification.basename;
                if pattern != "" {
                    //tfile
                    let tname_line = get_tfile_value(&pattern, &unwrapped_tfile);
                    t_string.push_str(&format!("\n{}", &tfile_ware(tbase, tname_line, &toml_parsed)));
                    //macro
                    macro_string = replace_pattern(&pattern, &macro_string, &format!("{{{},{}}}", &toml_parsed.config.pageid, tbase.to_string()));
                }
                let pattern = &macro_parsed.r#macro.properties.identification.description;
                if pattern != "" {
                    //tfile
                    let tname_line = get_tfile_value(&pattern, &unwrapped_tfile);
                    t_string.push_str(&format!("\n{}", &tfile_ware(tdesc, tname_line, &toml_parsed)));
                    //macro
                    macro_string = replace_pattern(&pattern, &macro_string, &format!("{{{},{}}}", &toml_parsed.config.pageid, tdesc.to_string()));
                }
                let pattern = &macro_parsed.r#macro.properties.identification.variation;
                if pattern != "" {
                    //tfile
                    let tname_line = get_tfile_value(&pattern, &unwrapped_tfile);
                    t_string.push_str(&format!("\n{}", &tfile_ware(tvar, tname_line, &toml_parsed)));
                    //macro
                    macro_string = replace_pattern(&pattern, &macro_string, &format!("{{{},{}}}", &toml_parsed.config.pageid, tvar.to_string()));
                }
                let pattern = &macro_parsed.r#macro.properties.identification.shortvariation;
                if pattern != "" {
                    //tfile
                    let tname_line = get_tfile_value(&pattern, &unwrapped_tfile);
                    t_string.push_str(&format!("\n{}", &tfile_ware(tshort, tname_line, &toml_parsed)));
                    //macro
                    macro_string = replace_pattern(&pattern, &macro_string, &format!("{{{},{}}}", &toml_parsed.config.pageid, tshort.to_string()));
                }
            }
            // common macro stuff
            // first order
            let mut cargo = 0;
            let mut mass = 0;
            let hull = "";
            // second order
            let ammo = "";
            let hangarcapacity = "";
            let people = "";

            //
            let i_pitch = "";
            let i_yaw = "";
            let i_roll = "";
            let forward = "";
            let reverse = "";
            let horizontal = "";
            let vertical = "";
            let d_pitch = "";
            let d_yaw = "";
            let d_roll = "";

            // let cargo = "";
            // let cargo = "";
            // let cargo = "";
            // let cargo = "";
            // let cargo = "";
            // let cargo = "";
            let purpose = &macro_parsed.r#macro.properties.purpose.primary;
            // println!("purpose {}", purpose);
            /*

            might.. want to check the odd assumption.... but hey 50/50... right?
            these ifs determine the ordered values and should eventually contain some unique logic beyond order
            a few points about the ordering:
            1. the order function should not be applied second order values as it loses its influence in longer chains of values.
                a. cargo can roll high to enforce a high mass which in turn can roll an essentially random hull
                b. a + or - values could be propagated through the chain by changing the average calculation to consider the min's
                    position relative to its range in min_and_value
            2. it would be fairly simple to add look ahead or look behind as some form of deterministic mechanism.
            3. rarity is derivable from the first order values.
                a. rarity can be split by tpwar faction type: major, minor, landed, aux, explore
                b. not sure how good it would be to split rarity by faction category: pirate, mercenary, trader, zealot, scavenger
            4.  the second order values are: ammo, people, hangarcapacity. the method described in 1.b might be the best method
            5. see toml comment on ordering for details

            purpose, mass, hull, ammo
            */
            if purpose == "build" {
                //mass
                let min = &toml_parsed.xlconfig.mass[0];
                let max = &toml_parsed.xlconfig.mass[1];
                let min_and_value = return_min_and_value(*min, *max);
                mass = min_and_value.1;
                // cargo
                let mut min = &toml_parsed.xlconfig.cargo[0];
                let mut max = &toml_parsed.xlconfig.cargo[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                cargo = min_and_value.1;
                // hull
                let mut min = &toml_parsed.xlconfig.hull[0];
                let mut max = &toml_parsed.xlconfig.hull[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    min = &average;
                } else {
                    max = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                let hull = min_and_value.1;
            }
            if purpose == "fight" {
                //hull
                let min = &toml_parsed.xlconfig.hull[0];
                let max = &toml_parsed.xlconfig.hull[1];
                let min_and_value = return_min_and_value(*min, *max);
                let hull = min_and_value.1;
                //mass
                let mut min = &toml_parsed.xlconfig.mass[0];
                let mut max = &toml_parsed.xlconfig.mass[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                mass = min_and_value.1;
                // cargo
                let mut min = &toml_parsed.xlconfig.cargo[0];
                let mut max = &toml_parsed.xlconfig.cargo[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                cargo = min_and_value.1;
            }
            if purpose == "trade" {
                // cargo
                let min = &toml_parsed.xlconfig.cargo[0];
                let max = &toml_parsed.xlconfig.cargo[1];
                let min_and_value = return_min_and_value(*min, *max);
                cargo = min_and_value.1;
                // mass
                let mut min = &toml_parsed.xlconfig.mass[0];
                let mut max = &toml_parsed.xlconfig.mass[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                mass = min_and_value.1;
                // hull
                let mut min = &toml_parsed.xlconfig.hull[0];
                let mut max = &toml_parsed.xlconfig.hull[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                let hull = min_and_value.1;
            }
            if purpose == "auxiliary" {
                // cargo
                let min = &toml_parsed.xlconfig.cargo[0];
                let max = &toml_parsed.xlconfig.cargo[1];
                let min_and_value = return_min_and_value(*min, *max);
                cargo = min_and_value.1;
                // hull
                let mut min = &toml_parsed.xlconfig.hull[0];
                let mut max = &toml_parsed.xlconfig.hull[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    min = &average;
                } else {
                    max = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                let hull = min_and_value.1;
                // mass
                let mut min = &toml_parsed.xlconfig.mass[0];
                let mut max = &toml_parsed.xlconfig.mass[1];
                let average = min + max / 2;
                if min_and_value.0 == 0 {
                    max = &average;
                } else {
                    min = &average
                }
                let min_and_value = return_min_and_value(*min, *max);
                mass = min_and_value.1;
            }
            let physics = format!(
                "<physics mass=\"{}\">
        <inertia pitch=\"{}\" yaw=\"{}\" roll=\"{}\"/>
        <drag forward=\"{}\" reverse=\"{}\" horizontal=\"{}\" vertical=\"{}\" pitch=\"{}\" yaw=\"{}\" roll=\"{}\"/>
      </physics>",
                mass, i_pitch, i_yaw, i_roll, forward, reverse, horizontal, vertical, d_pitch, d_yaw, d_roll
            );
            let re = Regex::new("((?s)<physics.*</physics>)").unwrap();
            macro_string = re.replace(&macro_string, physics.as_str()).into_owned();
            // hull replace
            let pattern = &macro_parsed.r#macro.properties.hull.max;
            if pattern != "" {
                replace_pattern(&pattern, &macro_string, hull);
            }
            let mut small = 0;
            if macro_string.contains("shipstorage_gen_s_01_macro") == true {
                let min = &toml_parsed.xlconfig.hangarcapacity[0];
                let max = &toml_parsed.xlconfig.hangarcapacity[1];
                let min_and_value = return_min_and_value(*min, *max);
                small = min_and_value.1;
                // replace name
                macro_string = macro_string.replace(
                    "shipstorage_gen_s_01_macro",
                    &macroname
                        .replace(".xml", "")
                        .replace("_macro", &[&toml_parsed.config.variant_name.as_str(), "size_s", "_macro"].concat())
                        .replace("ship", "shipstorage"),
                );
            }
            let mut medium = 0;
            if macro_string.contains("shipstorage_gen_m_01_macro") == true {
                let min = &toml_parsed.xlconfig.hangarcapacity[2];
                let max = &toml_parsed.xlconfig.hangarcapacity[3];
                let min_and_value = return_min_and_value(*min, *max);
                medium = min_and_value.1;
                //  replace name
                macro_string = macro_string.replace(
                    "shipstorage_gen_m_01_macro",
                    &macroname
                        .replace(".xml", "")
                        .replace("_macro", &[&toml_parsed.config.variant_name.as_str(), "size_m", "_macro"].concat())
                        .replace("ship", "shipstorage"),
                );
            }
            // table!
            macro_relations.insert(macroname.to_string(), (cargo.to_string(), small, medium));

            if macro_relations.contains_key(&macroname.to_owned().to_string().replace("ship", "storage")) {
                // oh this is either really smart or really dumb

                let medium = &macro_relations.get(&macroname.replace("storage", "ship").to_owned()).unwrap().2;
                if medium > &0 {
                    let size = "size_m";
                    makeshipstorage(&toml_parsed, &macroname.to_string(), &size.to_string(), &medium.to_string());
                }
                let small = &macro_relations.get(&macroname.replace("storage", "ship").to_owned()).unwrap().1;
                if small > &0 {
                    let size = "size_s";
                    makeshipstorage(&toml_parsed, &macroname.to_string(), &size.to_string(), &small.to_string());
                }
            }

            // ware

            let ware_string = fs::read_to_string(&toml_parsed.config.ware_path).unwrap();
            for ware in ware_string.split_terminator("</ware>") {
                if ware.contains(&macroname.replace(".xml", "")) == true {
                    let mut ware_new = "".to_string();

                    ware_new.push_str(&ware);

                    ware_new.push_str("\n</ware>");
                    let ware_parsed: Ware = serde_xml_rs::from_str(&ware_new).unwrap_or_default();
                    let mut prng = rand::thread_rng();
                    let ware_price = format!(
                        "<price min=\"{}\" average=\"{}\" max=\"{}\" />",
                        randomize(prng.gen_range(0.5, 5.0), 25000),
                        randomize(prng.gen_range(0.5, 5.0), 25000),
                        randomize(prng.gen_range(0.5, 5.0), 25000)
                    );
                    let re = Regex::new("<price.* />").unwrap();
                    let mut ware_new = re.replace(&ware_new, ware_price.as_str()).into_owned();

                    let pattern = &ware_parsed.name;
                    if pattern != "" {
                        ware_new = replace_pattern(&pattern, &ware_new, &format!("{{{},{}}}", &toml_parsed.config.pageid, tname.to_string()));
                    }
                    let pattern = &ware_parsed.description;
                    if pattern != "" {
                        ware_new = replace_pattern(&pattern, &ware_new, &format!("{{{},{}}}", &toml_parsed.config.pageid, tbase.to_string()));
                    }

                    ware_file_string.push_str("\n");
                    if variant == &true {
                        ware_file_string.push_str(&ware_new);
                    } else {
                        ware_file_string.push_str(&ware_new);
                    }
                }
            }

            output(&toml_parsed.config.out_path, &path, &toml_parsed.config.variant_name, &macro_string);
        }
    }

    let mut outputfile = File::create(format!("{}{}", &toml_parsed.config.out_path, "wares.xml")).unwrap();
    outputfile.write_all(ware_file_string.as_bytes()).unwrap();
    let mut outputfile = File::create(format!("{}{}", &toml_parsed.config.out_path, "tfiles.xml")).unwrap();
    outputfile.write_all(t_string.as_bytes()).unwrap();
}

fn makeshipstorage(toml_parsed: &Toml, macroname: &String, size: &String, count: &String) -> () {
    let shipstorage_string = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>
 <!--Exported by: Michael (192.168.3.150) at 09.11.2017_11-30-00-->
 <macros>
   <macro name=\"{}\" class=\"dockingbay\">
     <component ref=\"generic_dockingbay\" />
     <properties>
       <identification unique=\"0\" />
       <dock capacity=\"{}\" external=\"0\" storage=\"1\" />
       <room walkable=\"0\" />
       <docksize tags=\"{}\" />
     </properties>
   </macro>
 </macros>",
        &macroname
            .replace(".xml", "")
            .replace("_macro", &[&toml_parsed.config.variant_name.as_str(), size.as_str(), "_macro"].concat())
            .replace("storage", "shipstorage"),
        count,
        size
    );

    let mut outputfile = File::create(format!(
        "{}{}",
        &toml_parsed.config.out_path,
        &macroname
            .replace("storage", "shipstorage_small")
            .replace("_macro", &[&toml_parsed.config.variant_name.as_str(), size.as_str(), "_macro"].concat())
    ))
    .unwrap();
    outputfile.write_all(shipstorage_string.as_bytes()).unwrap();
}

// input min and max of expected range -> min or average of the range, and value of the range result.
fn return_min_and_value(min: i32, max: i32) -> (i32, i32) {
    let mut prng = rand::thread_rng();
    let mut returnmin = 0;
    let value = prng.gen_range(min, max);
    let average = min + max / 2;
    if value <= average {
        returnmin = average;
    }
    (returnmin, value)
}

fn replace_pattern(pattern: &String, text: &String, replace: &str) -> String {
    if pattern != "" {
        let text = &text.replace(pattern.as_str(), &replace);
        text.to_string()
    } else {
        text.to_string()
    }
}
// randomize(prng.gen_range(0.5, 5.0), 25000);
fn randomize(multi: f32, input: i32) -> String {
    let result = multi * input as f32;
    (result as i32).to_string()
}

fn tfile_ware(tnum: i32, tname_line: String, toml_parsed: &Toml) -> String {
    let mut tname_line = tname_line;
    let t_line_parsed: t = serde_xml_rs::from_str(&tname_line).unwrap_or_default();
    tname_line = tname_line.replace(&t_line_parsed.id, &tnum.to_string());
    tname_line = tname_line.replace(
        &t_line_parsed.content,
        &format!("{} {}", &t_line_parsed.content, &toml_parsed.config.variant_tname),
    );
    tname_line
}

fn output(path: &String, pathbuf: &std::path::PathBuf, variant: &String, macro_string: &String) {
    let mut outputfile = File::create(
        format!("{}{}", path, pathbuf.file_name().unwrap().to_str().unwrap().to_string()).replace("_macro", &[&variant.as_str(), "_macro"].concat()),
    )
    .unwrap();
    outputfile.write_all(macro_string.as_bytes()).unwrap();
}

// Alby's tfile stuff
// use t_path from Config toml and id string from parsed macro/ware
fn get_tfile_value(id_tfile: &String, unwrapped_tfile: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();
    let mut tfile_vec = vec![];
    for caps in re.captures_iter(&id_tfile) {
        let num = caps.get(0).unwrap().as_str();
        tfile_vec.push(num)
    }

    let mut tfile_value = "".to_string();
    let mut flag = false;
    for line in unwrapped_tfile.lines() {
        if flag == false {
            if line.contains(format!("<page id=\"{}", tfile_vec[0]).as_str()) {
                flag = true
            };
        } else {
            if line.contains(tfile_vec[1]) {
                tfile_value.push_str(line);
                break;
            };
        }
    }

    tfile_value
}
