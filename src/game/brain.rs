use bevy::{ecs::system::SystemState, prelude::*};

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_brains, preform_brain_actions).chain())
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
#[derive(Debug, Clone, Copy)]
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

pub struct GetFoodTask {
    current_action: BrainAction,
}

impl BrainTask for GetFoodTask {
    fn current_action(&self) -> Option<BrainAction> {
        Some(self.current_action)
    }

    fn update(&mut self, _world: &mut World, _brain: Entity) {
        self.current_action = BrainAction::UsingMachine;
    }
}

pub struct BuildWallTask {
    current_action: BrainAction,
}

impl BrainTask for BuildWallTask {
    fn current_action(&self) -> Option<BrainAction> {
        Some(self.current_action)
    }

    fn update(&mut self, world: &mut World, brain: Entity) {
        // Here we can get any combination of queries and resources
        let mut system: SystemState<Query<&Transform>> = SystemState::new(world);
        let transform_query = system.get(world);

        if let Ok(_brain_location) = transform_query.get(brain) {
            // Check location for example
            self.current_action = BrainAction::Build
        } else {
            // Brain has no transform
            self.current_action = BrainAction::Idle
        }
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
    commands.spawn((Brain {
        task: Box::new(GetFoodTask {
            current_action: BrainAction::Idle,
        }),
    },));

    commands.spawn((Brain {
        task: Box::new(BuildWallTask {
            current_action: BrainAction::Idle,
        }),
    },));
}

// TODO actually do actions
fn preform_brain_actions(brains: Query<(Entity, &Brain)>) {
    for (entity, brain) in &brains {
        info!("{:?}, {:?}", entity, brain.task.current_action());
    }
}
