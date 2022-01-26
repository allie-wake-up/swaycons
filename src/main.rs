use regex::RegexSet;
use std::collections::HashMap;
use swaycons::settings::{get_settings, BaseSettings, GlobalSettings, TitleSettings};
use swayipc::{Connection, Event, EventType, Fallible};

const DEBUG: bool = false;

fn main() -> Fallible<()> {
    let settings = get_settings().unwrap();
    let global_settings = settings.global;
    let title_settings = settings.title;
    let app_id_settings = settings.app_id;
    let title_set = RegexSet::new(title_settings.keys()).unwrap();
    let mut ignore: HashMap<i64, String> = HashMap::new();
    let mut last_focused = None;
    let mut last_focused_settings: Option<&BaseSettings> = None;
    let mut connection = Connection::new()?;
    for event in Connection::new()?.subscribe([EventType::Window])? {
        if let Event::Window(w) = event? {
            let id = w.container.id;
            let app_id = match w.container.window_properties {
                Some(properties) => properties.class.unwrap_or_default(),
                None => w.container.app_id.unwrap_or_default(),
            };
            let focused = w.container.focused;
            let name = w.container.name.unwrap_or_default();
            let ignore_entry = ignore.entry(id).or_default();
            if DEBUG {
                println!("id: {}, app_id: {}, name: {}", id, app_id, name);
            }
            match find_best_match(&title_set, &name, &title_settings, &app_id) {
                Some(index) => {
                    if let Some(pattern) = title_set.patterns().get(index) {
                        if ignore_entry != pattern || (focused && !(last_focused == Some(id))) {
                            let settings = title_settings.get(pattern).unwrap();
                            ignore.insert(id, pattern.to_owned());
                            if focused {
                                if let Some(last_id) = last_focused {
                                    if let Some(last_settings) = last_focused_settings {
                                        set_icon(
                                            last_id,
                                            last_settings,
                                            &global_settings,
                                            &mut connection,
                                            false,
                                        )?;
                                    }
                                }
                                last_focused = Some(id);
                                last_focused_settings = Some(&settings.base);
                            }
                            set_icon(
                                id,
                                &settings.base,
                                &global_settings,
                                &mut connection,
                                focused,
                            )?;
                        }
                    }
                }
                None => {
                    if let Some(settings) = app_id_settings.get(&app_id) {
                        if ignore_entry != &app_id || (focused && !(last_focused == Some(id))) {
                            ignore.insert(id, app_id);
                            if focused {
                                if let Some(last_id) = last_focused {
                                    if let Some(last_settings) = last_focused_settings {
                                        set_icon(
                                            last_id,
                                            last_settings,
                                            &global_settings,
                                            &mut connection,
                                            false,
                                        )?;
                                    }
                                }
                                last_focused = Some(id);
                                last_focused_settings = Some(&settings);
                            }
                            set_icon(id, settings, &global_settings, &mut connection, focused)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn find_best_match(
    title_set: &RegexSet,
    name: &str,
    title_settings: &HashMap<String, TitleSettings>,
    app_id: &String,
) -> Option<usize> {
    let matches = title_set.matches(name);
    let mut best_match = None;
    for m in matches.into_iter() {
        let current = title_set.patterns().get(m).unwrap();
        let settings = title_settings.get(current).unwrap();
        if let Some(app_id_match) = settings.app_id.as_ref() {
            if !app_id_match.contains(app_id) {
                continue;
            }
        }

        if best_match == None {
            best_match = Some(m);
        } else {
            let best = title_set.patterns().get(best_match.unwrap()).unwrap();
            let current = title_set.patterns().get(m).unwrap();
            if current.contains(best) {
                best_match = Some(m);
            }
        }
    }
    best_match
}

fn set_icon(
    id: i64,
    settings: &BaseSettings,
    global_settings: &GlobalSettings,
    connection: &mut Connection,
    focused: bool,
) -> Fallible<()> {
    let color = if focused && global_settings.focused_color.is_some() {
        global_settings.focused_color.as_ref().unwrap()
    } else {
        settings.color.as_ref().unwrap_or(&global_settings.color)
    };
    let icon = settings.icon.as_ref().unwrap_or(&global_settings.icon);
    let size = settings.size.as_ref().unwrap_or(&global_settings.size);
    connection.run_command(format!(
        "[con_id={}] title_format \"<span color='{}' size='{}'>{}</span> %title\"",
        id, color, size, icon
    ))?;
    Ok(())
}
