use std::rc::Rc;

use bladeink::story::Story;

use crate::game::dialog_system::DialogChoice;

pub fn get_story_with_state(inkjson: &str, state: &str, knot: &str) -> Story {
    let mut story = match Story::new(inkjson) {
        Ok(story) => story,
        Err(err) => panic!("Story can't be read: {:?}", err),
    };

    if !state.is_empty() {
        story.load_state(state).expect("Could not load story state");
    }

    if !knot.is_empty() {
        story
            .choose_path_string(knot, true, None)
            .expect("Could not load story knot");
    }

    story
}

pub fn get_lines(story: &mut Story) -> Vec<String> {
    let mut lines = Vec::new();

    while story.can_continue() {
        if let Ok(line) = story.cont() {
            lines.push(line);
        }
    }

    lines
}

pub fn get_choices(story: &Story) -> Vec<DialogChoice> {
    let mut choices = vec![];

    for mut choice in story.get_current_choices().into_iter() {
        let choice = Rc::make_mut(&mut choice);

        choices.push(DialogChoice {
            body: choice.text.clone(),
            index: choice.index.clone().into_inner(),
        });
    }

    choices
}
