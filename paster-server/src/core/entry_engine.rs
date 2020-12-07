use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn uuid_generator() -> String {
    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    return rand_string;
}

fn load_value_by_uuid(uuid: String) -> Entry {
    Entry {
        uuid: uuid,
        value: "",
        time_to_live_in_minutes: 0,
    }
}

fn delete_entry(uuid: String) -> bool {
    true
}

fn save_entry(entry: Entry) -> bool {
    true
}

fn save_entry(uuid: String, entry: Entry) -> bool {
    true
}
