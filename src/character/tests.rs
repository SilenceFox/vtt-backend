use super::{skills::Skills, *};

#[test]
fn add_skill() {
    let mut skill = Skills::default();
    assert!(skill.0.contains_key(&Arc::from("Shoot")));
    skill.add_skill("Swordsmanship");
    assert!(skill.0.contains_key(&Arc::from("Swordsmanship")));
}
#[test]
#[should_panic]
fn delete_skill() {
    let mut skill = Skills::default();
    assert!(skill.0.contains_key(&Arc::from("Shoot")));
    skill
        .add_skill("Swordsmanship")
        .remove_skill("Swordsmanship");
    assert!(skill.0.contains_key(&Arc::from("Swordsmanship")));
}
//Test that should fail
#[test]
#[should_panic]
fn increment_skill_fail() {
    let mut skill = Skills::default();
    skill.increment_skill("Swordsmanship");
    assert!(skill.0.get_key_value("Swordsmanship").is_some());
}

#[test]
fn increment_skill() {
    let mut skill = Skills::default();
    skill
        .add_skill("Swordsmanship")
        .increment_skill("Swordsmanship");
    assert!(skill.0.get_key_value("Swordsmanship").is_some());
}

#[test]
fn increment_two_times() {
    let mut skill = Skills::default();
    skill.increment_skill("Shoot").increment_skill("Shoot");
    assert_eq!(skill.0.get("Shoot"), Some(2).as_ref());
}

#[test]
fn get_skill_value() {
    let mut skill = Skills::default();
    skill.add_skill("Shoot").increment_skill("Shoot");
    assert_eq!(skill.0.get("Shoot"), Some(1).as_ref());
}
