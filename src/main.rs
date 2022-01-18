use regex::RegexSet;
use std::collections::HashMap;
use swaycons::settings::{get_settings, BaseSettings, GlobalSettings};
use swayipc::{Connection, Event, EventType, Fallible};

const DEBUG: bool = false;

fn main() -> Fallible<()> {
    let settings = get_settings().unwrap();
    let global_settings = settings.global;
    let title_settings = settings.title;
    let app_id_settings = settings.app_id;
    let title_set = RegexSet::new(title_settings.keys()).unwrap();
    let mut ignore: HashMap<i64, String> = HashMap::new();
    let mut connection = Connection::new()?;
    for event in Connection::new()?.subscribe([EventType::Window])? {
        if let Event::Window(w) = event? {
            let id = w.container.id;
            let app_id = match w.container.window_properties {
                Some(properties) => properties.class.unwrap_or_default(),
                None => w.container.app_id.unwrap_or_default(),
            };
            let name = w.container.name.unwrap_or_default();
            let ignore_entry = ignore.entry(id).or_default();
            if DEBUG {
                println!("id: {}, app_id: {}, name: {}", id, app_id, name);
            }
            match title_set.matches(name.as_str()).iter().next() {
                Some(index) => {
                    if let Some(pattern) = title_set.patterns().get(index) {
                        if ignore_entry != pattern {
                            let settings = title_settings.get(pattern).unwrap();
                            match settings.app_id.as_ref() {
                                Some(app_id_match) => {
                                    if app_id_match.contains(&app_id) {
                                        ignore.insert(id, pattern.to_owned());
                                        set_icon(
                                            id,
                                            &settings.base,
                                            &global_settings,
                                            &mut connection,
                                        )?;
                                    } else if let Some(settings) = app_id_settings.get(&app_id) {
                                        if ignore_entry != &app_id {
                                            ignore.insert(id, app_id);
                                            set_icon(
                                                id,
                                                settings,
                                                &global_settings,
                                                &mut connection,
                                            )?;
                                        }
                                    }
                                }
                                None => {
                                    ignore.insert(id, pattern.to_owned());
                                    set_icon(
                                        id,
                                        &settings.base,
                                        &global_settings,
                                        &mut connection,
                                    )?;
                                }
                            }
                        }
                    }
                }
                None => {
                    if let Some(settings) = app_id_settings.get(&app_id) {
                        if ignore_entry != &app_id {
                            ignore.insert(id, app_id);
                            set_icon(id, settings, &global_settings, &mut connection)?;
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
    settings: &BaseSettings,
    global_settings: &GlobalSettings,
    connection: &mut Connection,
) -> Fallible<()> {
    let color = settings.color.as_ref().unwrap_or(&global_settings.color);
    let icon = settings.icon.as_ref().unwrap_or(&global_settings.icon);
    let size = settings.size.as_ref().unwrap_or(&global_settings.size);
    connection.run_command(format!(
        "[con_id={}] title_format \"<span color='{}' size='{}'>{}</span> %title\"",
        id, color, size, icon
    ))?;
    Ok(())
}
