use swayipc::{Connection, Event, EventType, Fallible, WindowChange};
use regex::RegexSet;

fn main() -> Fallible<()> {
    let set = RegexSet::new(&[
        r"google\.com",
    ]).unwrap();
    let mut ignore = vec![];
    let mut connection = Connection::new()?;
    for event in Connection::new()?.subscribe([EventType::Window])? {
        if let Event::Window(w) = event? {
            if w.change == WindowChange::Title {
                let id = w.container.id;
                if let Some(app_id) = w.container.app_id {
                    if let Some(name) = w.container.name {
                        println!("{:?}", ignore);
                        println!("id: {}, app_id: {}, name: {}", id, app_id, name);
                        if let Some(index) = set.matches(name.as_str()).iter().next() {
                            match ignore.iter().position(|&x: &i64| x == id) {
                                Some(i) => {
                                    ignore.swap_remove(i as usize);
                                    ()
                                },
                                None => {
                                    ignore.push(id);
                                    connection.run_command(format!("[title={}] title_format \"%title\"", name))?;
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
