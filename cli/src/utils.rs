pub fn action_from_str(s: &str) -> esc_api::access::Action {
    use esc_api::access::Action;
    match s {
        "create" => Action::Create,
        "delete" => Action::Delete,
        "modify" => Action::Modify,
        "none" => Action::_None,
        "read" => Action::Read,
        _ => {
            eprintln!("Unknown action: {}", s);
            std::process::exit(1);
        }
    }
}

pub fn actions_from_str_vec(strings: Vec<String>) -> Vec<esc_api::access::Action> {
    strings
        .iter()
        .map(|s| action_from_str(s.as_ref()))
        .collect()
}

pub fn effect_from_str(s: &str) -> esc_api::access::Effect {
    use esc_api::access::Effect;
    match s {
        "allow" => Effect::Allow,
        "deny" => Effect::Deny,
        _ => {
            eprintln!("Unknown effect: {}", s);
            std::process::exit(1);
        }
    }
}
