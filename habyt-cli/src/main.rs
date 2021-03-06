extern crate lib;
#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use lib::{HabitDraft, HabitId, HabitLog, HabitPatch, HabitStoreFs};

fn main() {
    let matches = app_from_crate!()
        .template("{bin} {version}\n{author}\n\n{about}\n\nUSAGE:\n    {usage}\n\nFLAGS:\n{flags}\n\nSUBCOMMANDS:\n{subcommands}")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new habit")
                .arg(
                    Arg::with_name("name")
                        .index(1)
                        .help("A one or two word name for the habit. E.g. writing")
                        .required(true)
                    )
                .arg(
                    Arg::with_name("quantum")
                        .index(2)
                        .help("A daily goal. E.g. 750")
                        .required(true)
                    )
                .arg_from_usage("-u, --unit=[unit] 'A measurable unit. E.g. words'")
        )
        .subcommand(
            SubCommand::with_name("upd")
                .about("Update a habit")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .help("'id' of the habit you want to update . E.g. 1")
                        .required(true)
                    )
                .arg_from_usage("-n, --name=[name] 'A one or two word name for the habit. E.g. writing'")
                .arg_from_usage("-q, --quantum=[quantum] 'A daily goal. E.g. 750'")
        )
        .subcommand(
            SubCommand::with_name("del")
                .about("Delete a habit")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .help("'id' of the habit you want to delete . E.g. 1")
                        .required(true)
                    )
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Log a habit")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .help("'id' of the habit you want to log . E.g. 1")
                        .required(true)
                    )
                .arg_from_usage("-q, --quantum=[quantum] 'The goal youve been able to achieve today. E.g. 750'")
                .arg_from_usage("-n, --notes=[notes] 'Any accompanying thoughts youd like to add.'")
        )
        .subcommand(SubCommand::with_name("list").about("List all the habits"))
        .subcommand(SubCommand::with_name("llist").about("List all the log"))
        .get_matches();

    //
    let mut habitctl = HabitStoreFs::new();
    habitctl.load();

    //
    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let name = String::from(sub_matches.value_of("name").unwrap());
            let quantum = value_t!(sub_matches, "quantum", f64).unwrap();
            let unit = String::from(sub_matches.value_of("unit").unwrap_or(""));

            let habit_id: HabitId =
                habitctl
                    .store
                    .create(HabitDraft::new(name.clone(), quantum, unit.clone()));

            habitctl.save();

            println!(
                "You have commited to {} {} of {} ({}) every day!",
                &quantum, &unit, &name, habit_id
            );
        }
        ("upd", Some(sub_matches)) => {
            let id: u32 = String::from(sub_matches.value_of("id").unwrap())
                .parse::<u32>()
                .unwrap();

            habitctl.store.update(
                id,
                HabitPatch::from(
                    String::from(sub_matches.value_of("name").unwrap_or("")),
                    value_t!(sub_matches, "quantum", f64).unwrap_or(0.),
                    String::from(sub_matches.value_of("unit").unwrap_or("")),
                    None,
                    None,
                    None,
                    None,
                ),
            );

            habitctl.save();
        }
        ("del", Some(sub_matches)) => {
            let id: u32 = String::from(sub_matches.value_of("id").unwrap())
                .parse::<u32>()
                .unwrap();
            if let Some(habit) = habitctl.store.delete(id) {
                habitctl.save();
                println!(
                    "Habyt is not tracking {} with id {} anymore!",
                    &habit.0.name, &habit.0.id
                );
            }
        }
        ("list", Some(_)) => {
            let habits = habitctl.store.list();

            println!("You've commited to {} habits so far...", habits.len());
            for habit in habits.iter() {
                println!(
                    "-> {} for {} {} a day \n",
                    habit.name, habit.quantum, habit.unit
                );
            }
        }
        ("log", Some(sub_matches)) => {
            let id: u32 = String::from(sub_matches.value_of("id").unwrap())
                .parse::<u32>()
                .unwrap();

            habitctl.load_log();
            habitctl.store_log.add(HabitLog::new(
                id,
                value_t!(sub_matches, "quantum", f64).unwrap_or(0.),
                Some(String::from(sub_matches.value_of("notes").unwrap_or(""))),
            ));
            habitctl.save_log();
        }
        ("llist", Some(_)) => {
            habitctl.load_log();
            let logs = habitctl.store_log.get_log_by_habit();
            // let quantum: Vec<f64> = vec![];

            // println!("You've made {} entries so far...", logs.len());
            for (id, log_entries) in logs.iter() {
                println!("-> {} for a day \n", id);
                for (log_entry_date, log_entry_quantum) in log_entries.iter() {
                    println!("-> {} for a {} \n", log_entry_date, log_entry_quantum);
                }
            }
        }
        _ => {
            // no subcommand used
        }
    }
}
