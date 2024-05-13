enum DoorState {
  Opened,
  Closed
}

enum DoorAction {
  Open,
  Close
}

fn take_action(state: DoorState, action: DoorAction) {
  match (state, action) {
    (DoorState::Opened, DoorAction::Close) => println!("Closing door.."),
    (DoorState::Closed, DoorAction::Open) => println!("Opening door.."),
    _ => unreachable!()
  }
}

fn main() {
  // Valid actions.
  take_action(DoorState::Closed, DoorAction::Open);
  take_action(DoorState::Opened, DoorAction::Close);

  // Invalid actions.
  take_action(DoorState::Closed, DoorAction::Close);
  take_action(DoorState::Opened, DoorAction::Open);
}
