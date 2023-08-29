use std::{any::Any, time::Duration};

use bevy::prelude::*;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct BrainSystemSet;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, BrainSystemSet.before(update_brains))
            .add_systems(
                Update,
                (update_get_food_tasks, update_build_wall_tasks).in_set(BrainSystemSet),
            )
            .add_systems(Update, update_brains)
            .add_systems(Startup, spawn_dummy_brains);
    }
}

#[derive(Component)]
pub struct Brain {
    // This is what the unit is currently doing, should be resolved into frame by frame action
    task: Box<dyn BrainTask>,
    // This is the larger picture for the brain (ie if I'm trying to get food but my current action is walking)
    context: Context,
}

// The things that the brain can do
// Other systems should detect and move the unit according to these, set by task.current_action
#[derive(Component, Debug)]
pub enum BrainAction {
    Idle,
    WaitingOnPathfinding,
    Walking,
    UsingMachine,
    PickupItem,
    Build,
}

// A list of actions to complete a task
pub trait BrainTask: Send + Sync {
    fn current_action(&self) -> Option<BrainAction>;
    fn update(&mut self, context: &Context);
    // This can't be a default impl for reasons beyond me
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct GetFoodTask {
    // In some cases could just store the current action as the enum varient
    current_action: usize,
    in_range_of_food_machine: bool,
}

impl BrainTask for GetFoodTask {
    fn current_action(&self) -> Option<BrainAction> {
        match self.current_action {
            0 => Some(BrainAction::WaitingOnPathfinding),
            1 => Some(BrainAction::Walking),
            2 => Some(BrainAction::UsingMachine),
            _ => None,
        }
    }

    fn update(&mut self, context: &Context) {
        if self.in_range_of_food_machine {
            if context.hungry {
                self.current_action = 2;
            } else {
                // Done with the task. Better way to signal?
                self.current_action = 9999;
            }
        } else {
            self.current_action = 1;
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct BuildWallTask {
    current_action: usize,
    last_frame_time: Duration,
    timer: Timer,
}

impl BrainTask for BuildWallTask {
    fn current_action(&self) -> Option<BrainAction> {
        match self.current_action {
            0 => Some(BrainAction::WaitingOnPathfinding),
            1 => Some(BrainAction::Walking),
            2 => Some(BrainAction::PickupItem),
            3 => Some(BrainAction::WaitingOnPathfinding),
            4 => Some(BrainAction::Walking),
            5 => Some(BrainAction::Build),
            _ => None,
        }
    }

    fn update(&mut self, _context: &Context) {
        if self.timer.tick(self.last_frame_time).just_finished() {
            self.current_action += 1;
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default, Clone)]
pub struct Context {
    // Goal tracking or something?
    // Needs?
    hungry: bool,
}

fn update_brains(mut brains: Query<(&mut Brain, &mut BrainAction)>) {
    for (mut brain, mut action) in &mut brains {
        // bless the borrow checker
        let context = brain.context.clone();
        brain.task.update(&context);
        // Is this really needed??
        if let Some(new_action) = brain.task.current_action() {
            info!("Working on {:?}", new_action);
            *action = new_action;
        } else {
            // Task complete! Find a new task
            // How should this be signaled
            *action = BrainAction::Idle;
        }
    }
}

fn spawn_dummy_brains(mut commands: Commands) {
    commands.spawn((
        Brain {
            task: Box::new(BuildWallTask {
                current_action: 0,
                last_frame_time: Duration::default(),
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            }),
            context: Context::default(),
        },
        BrainAction::Idle,
    ));
}

fn update_get_food_tasks(mut brains: Query<&mut Brain>, _food_machine_dummy_query: Query<()>) {
    for mut brain in &mut brains {
        if let Some(food_task) = brain.task.as_any().downcast_mut::<GetFoodTask>() {
            // Could check for food machine distance here
            food_task.in_range_of_food_machine = true;
        }
    }
}

fn update_build_wall_tasks(mut brains: Query<&mut Brain>, time: Res<Time>) {
    for mut brain in &mut brains {
        if let Some(build_wall_task) = brain.task.as_any().downcast_mut::<BuildWallTask>() {
            build_wall_task.last_frame_time = time.delta();
        }
    }
}
