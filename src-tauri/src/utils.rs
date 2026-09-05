use std::vec;
use freedesktop_desktop_entry::{default_paths, get_languages_from_env, Iter};
use freedesktop_icons::{lookup};
use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct AppInfo {
    name:  String,
    exec: String,
    desc: String,
    icon: String,
}

pub fn get_apps() -> HashMap<String, Vec<AppInfo>> {
    let locales = get_languages_from_env();
    let mut apps_organized: HashMap<&str, Vec<AppInfo>> = HashMap::from([
        ("Audio and Video", vec![]),
        ("Development", vec![]),
        ("Education", vec![]),
        ("Game", vec![]),
        ("Graphics", vec![]),
        ("Health and Fitness", vec![]),
        ("Network", vec![]),
        ("Office", vec![]),
        ("Science", vec![]),
        ("System", vec![]),
        ("Settings", vec![]),
        ("Utility", vec![])

    ]);

    for entry in Iter::new(default_paths()).entries(Some(&locales)).collect::<Vec<_>>() {
        let name = entry.name(&locales);
        let name = name.as_deref();
        let name = name.map(String::from).unwrap_or_default();

        let desc = entry.comment(&locales);
        let desc = desc.as_deref();
        let desc = desc.map(String::from);


        let exec = entry.exec();
        let exec = exec.unwrap_or("echo 'nothing to execute'").to_string();

        let categories = entry.categories().unwrap_or_default();

        let icon_name = entry.icon().unwrap_or("default");
        let dont_show = entry.no_display();

        if dont_show {
            continue;
        }
        
        if let Some(path) = lookup(icon_name).find() {
            println!("{:?}", categories);
            let mut health_checked = false;
            for catagory in categories {
                let str_path: String = path.to_string_lossy().into_owned();
                if ["Audio".to_string(), "Video".to_string(), "AudioVideo".to_string()].iter().all(|cat| catagory == cat) {
                    if let Some(vect) = apps_organized.get_mut("Audio and Video") {
                        if health_checked {
                            continue;
                        }
                        health_checked = true;
                        vect.push(AppInfo {
                            name: (name.clone()),
                            exec: (exec.clone()),
                            desc: (desc.clone().unwrap_or("no discription provided".into())),
                            icon: (str_path)
                        });
                    } else {
                        println!("Unexpected, AudioVideo type not found in HashMap");
                    }
                }
                else if catagory == "HealthFitness" {
                    if let Some(vect) = apps_organized.get_mut("Health and Fitness") {
                        vect.push(AppInfo {
                            name: (name.clone()),
                            exec: (exec.clone()),
                            desc: (desc.clone().unwrap_or("no discription provided".into())),
                            icon: (str_path)
                        });
                    } else {
                        println!("Unexpected, HealthFitness type not found in HashMap");
                    }
                }
                else if apps_organized.contains_key(catagory) {
                    if let Some(vect) = apps_organized.get_mut(catagory) {
                        vect.push(AppInfo {
                            name: (name.clone()),
                            exec: (exec.clone()),
                            desc: (desc.clone().unwrap_or("no discription provided".into())),
                            icon: (str_path)
                        });
                    } else {
                        // this is just a additional catagory, skipable
                    }
                }
            }

        }

    }

    apps_organized.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

#[derive(serde::Serialize)]
pub struct NocturaShell {
    workplaces: u16,
    current_workplace: u8,
    theme: String,
}