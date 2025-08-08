use crate::target::{self, Target};
use std::{collections::HashMap, hash::RandomState};

/*
Here are some additional functionalities you could consider adding to your `State` struct:

DONE (target_list, and selected_target_identifier) 1. **Current Target Tracking**: Keep track of the currently active target file.
2. **Undo/Redo History**: Maintain a history of changes for undo/redo functionality.
3. **Session Metadata**: Store metadata like timestamps, user preferences, or session-specific configurations.
4. **Error Logging**: Include a mechanism to log errors or warnings encountered during operations.
5. **Search and Filter**: Add functionality to search or filter through the imported target files.
6. **State Persistence**: Allow saving and loading the state to/from a file for session continuity.
7. **Concurrency Support**: If applicable, manage concurrent access to the state.
8. **Validation**: Include methods to validate the integrity of the state or imported files.
9. **Event System**: Add an event system to notify other parts of the program when the state changes.
10. **Statistics**: Track statistics like the number of targets, last modified time, etc.
*/

//STILL NEEDS LOTS OF FUNCTIONALITY LIKE '''IMPL STATE''' AND MORE IMPL STUFF
pub struct State {
    pub target_list: HashMap<String, Target>,
    pub selected_target_identifier: String, //might be depricated, dont know a use for it yet
}

impl State {
    fn new() -> Self {
        Self {
            target_list: HashMap::new(),
            selected_target_identifier: String::new()
        }
    }
    fn add_target(mut self, new_target: Target, target_identifier: String) {
        self.target_list.insert(
            target_identifier,
            new_target
        );
    }
}