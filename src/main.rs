use regex::RegexSet;
use std::collections::HashMap;
use swaycons::get_settings;
use swayipc::{Connection, Event, EventType, Fallible, WindowChange};

fn main() -> Fallible<()> {
    let settings = get_settings().unwrap();
    let global_color = settings.get_str("global.color").unwrap();
    let global_icon = settings.get_str("global.icon").unwrap();
    let global_size = settings.get_str("global.size").unwrap();
    let titles: HashMap<String, HashMap<String, String>> = settings.get("title").unwrap();
    let set = RegexSet::new(titles.keys()).unwrap();
    let mut ignore: HashMap<i64, String> = HashMap::new();
    let mut connection = Connection::new()?;
    for event in Connection::new()?.subscribe([EventType::Window])? {
        if let Event::Window(w) = event? {
            if w.change == WindowChange::Title {
                let id = w.container.id;
                if let Some(app_id) = w.container.app_id {
                    if let Some(name) = w.container.name {
                        if let Some(index) = set.matches(name.as_str()).iter().next() {
                            if let Some(pattern) = set.patterns().get(index) {
                                let stuff = titles.get(pattern).unwrap();
                                let entry = ignore.entry(id).or_default();
                                if entry != pattern {
                                    ignore.insert(id, pattern.to_owned());
                                    let color = stuff.get("color").unwrap_or(&global_color);
                                    let icon = stuff.get("icon").unwrap_or(&global_icon);
                                    let size = stuff.get("size").unwrap_or(&global_size);
                                    connection.run_command(format!("[con_id={}] title_format \"<span color='{}' size='{}'>{}</span> %title\"", id, color, size, icon))?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
