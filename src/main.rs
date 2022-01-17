use regex::RegexSet;
use std::collections::HashMap;
use swaycons::get_settings;
use swayipc::{Connection, Event, EventType, Fallible, WindowChange};

#[derive(Clone, Debug)]
struct GlobalSettings {
    color: String,
    icon: String,
    size: String,
}

impl GlobalSettings {
    pub fn new(mut config: HashMap<String, String>) -> GlobalSettings {
        let color = config.remove("color").unwrap();
        let icon = config.remove("icon").unwrap();
        let size = config.remove("size").unwrap();
        GlobalSettings { color, icon, size }
    }
}

fn main() -> Fallible<()> {
    let settings = get_settings().unwrap();
    let global: HashMap<String, String> = settings.get("global").unwrap();
    let global_settings = GlobalSettings::new(global);
    let title_config: HashMap<String, HashMap<String, String>> = settings.get("title").unwrap();
    let app_id_config: HashMap<String, HashMap<String, String>> = settings.get("app_id").unwrap();
    let title_set = RegexSet::new(title_config.keys()).unwrap();
    let mut ignore: HashMap<i64, String> = HashMap::new();
    for event in Connection::new()?.subscribe([EventType::Window])? {
        if let Event::Window(w) = event? {
            if w.change == WindowChange::Title {
                let id = w.container.id;
                let app_id = w.container.app_id.unwrap_or_default();
                let name = w.container.name.unwrap_or_default();
                let ignore_entry = ignore.entry(id).or_default();
                match title_set.matches(name.as_str()).iter().next() {
                    Some(index) => {
                        if let Some(pattern) = title_set.patterns().get(index) {
                            if ignore_entry != pattern {
                                let config = title_config.get(pattern).unwrap();
                                match config.get("app_id") {
                                    Some(app_id_match) => {
                                        if app_id_match == &app_id {
                                            ignore.insert(id, pattern.to_owned());
                                            set_icon(id, config, &global_settings)?;
                                        } else if let Some(config) = app_id_config.get(&app_id) {
                                            if ignore_entry != &app_id {
                                                ignore.insert(id, app_id);
                                                set_icon(id, config, &global_settings)?;
                                            }
                                        }
                                    }
                                    None => {
                                        ignore.insert(id, pattern.to_owned());
                                        set_icon(id, config, &global_settings)?;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        if let Some(config) = app_id_config.get(&app_id) {
                            if ignore_entry != &app_id {
                                ignore.insert(id, app_id);
                                set_icon(id, config, &global_settings)?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn set_icon(
    id: i64,
    config: &HashMap<String, String>,
    global_settings: &GlobalSettings,
) -> Fallible<()> {
    let mut connection = Connection::new()?;
    let color = config.get("color").unwrap_or(&global_settings.color);
    let icon = config.get("icon").unwrap_or(&global_settings.icon);
    let size = config.get("size").unwrap_or(&global_settings.size);
    connection.run_command(format!(
        "[con_id={}] title_format \"<span color='{}' size='{}'>{}</span> %title\"",
        id, color, size, icon
    ))?;
    Ok(())
}
