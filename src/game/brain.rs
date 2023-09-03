use bevy::prelude::*;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_brains)
            .add_systems(Startup, spawn_dummy_brains);
    }
}

#[derive(Component)]
pub struct Brain {
    // This is what the unit is currently doing, should be resolved into frame by frame action
    task: Box<dyn BrainTask>,
}

// The things that the brain can do
// Other systems should detect and move the unit according to these, set by task.current_action
#[derive(Component, Debug, Clone, Copy)]
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
    fn update(&mut self, world: &mut World, brain: Entity);
}

#[derive(Clone)]
pub struct GetFoodTask {
    current_action: BrainAction,
}

impl BrainTask for GetFoodTask {
    fn current_action(&self) -> Option<BrainAction> {
        Some(self.current_action)
    }

    fn update(&mut self, _world: &mut World, _brain: Entity) {
        self.current_action = BrainAction::Idle;
    }
}

fn update_brains(world: &mut World) {
    let mut to_update = Vec::default();
    let mut brains = world.query::<(Entity, &mut Brain)>();

    // Steal all the tasks
    for (entity, mut brain) in brains.iter_mut(world) {
        let dummy_task = Box::new(DummyTask); // DummyTask should implement BrainTask
        let task = std::mem::replace(&mut brain.task, dummy_task);
        to_update.push((task, entity));
    }

    // Update each
    for (task, entity) in &mut to_update {
        task.update(world, *entity);
    }

    // Return the tasks to the brains
    let mut brains = world.query::<&mut Brain>();
    for (mut brain, task) in brains.iter_mut(world).zip(to_update.into_iter()) {
        let _ = std::mem::replace(&mut brain.task, task.0);
    }
}

pub struct DummyTask;

impl BrainTask for DummyTask {
    fn current_action(&self) -> Option<BrainAction> {
        todo!()
    }

    fn update(&mut self, _world: &mut World, _brain: Entity) {
        todo!()
    }
}

fn spawn_dummy_brains(mut commands: Commands) {
    commands.spawn((
        Brain {
            task: Box::new(GetFoodTask {
                current_action: BrainAction::Build,
            }),
        },
        BrainAction::Idle,
    ));

    commands.spawn((
        Brain {
            task: Box::new(GetFoodTask {
                current_action: BrainAction::Build,
            }),
        },
        BrainAction::Idle,
    ));
}
