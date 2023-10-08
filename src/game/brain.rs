use bevy::{ecs::system::SystemState, prelude::*};
use rand::Rng;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_brains, preform_brain_actions).chain());
    }
}

#[derive(Component)]
pub struct Brain {
    // This is what the unit is currently doing, should be resolved into frame by frame action
    task: Box<dyn BrainTask>,
}

impl Brain {
    pub fn new() -> Self {
        Brain {
            task: Box::<WanderTask>::default(),
        }
    }
}

// The things that the brain can do
// Other systems should detect and move the unit according to these, set by task.current_action
#[derive(Debug, Clone, Copy)]
pub enum BrainAction {
    Idle,
    WaitingOnPathfinding,
    Walking(Vec2),
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

pub struct WanderTask {
    current_action: BrainAction,
}

impl Default for WanderTask {
    fn default() -> Self {
        Self {
            current_action: BrainAction::Idle,
        }
    }
}

impl BrainTask for WanderTask {
    fn current_action(&self) -> Option<BrainAction> {
        Some(self.current_action)
    }

    fn update(&mut self, world: &mut World, brain: Entity) {
        if let BrainAction::Walking(target) = self.current_action {
            let current_transform = world.get::<Transform>(brain).unwrap();
            // FIXME based on speed and frame rate
            if current_transform.translation.truncate().distance(target) < 0.3 {
                let (x, y) = rand::thread_rng().gen::<(f32, f32)>();
                self.current_action = BrainAction::Walking(Vec2::new(x * 20.0, y * 20.0));
            }
        } else {
            let (x, y) = rand::thread_rng().gen::<(f32, f32)>();
            self.current_action = BrainAction::Walking(Vec2::new(x * 20.0, y * 20.0));
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
        // pls dont destroy brain
        task.update(world, *entity);
    }

    // Return the tasks to the brains
    let mut brains = world.query::<&mut Brain>();
    // does this always work... do the brains iter in the same order? we are an exclusive system so probably but maybe not lmao
    for (mut brain, task) in brains.iter_mut(world).zip(to_update.into_iter()) {
        let _ = std::mem::replace(&mut brain.task, task.0);
    }
}

pub struct DummyTask;

impl BrainTask for DummyTask {
    fn current_action(&self) -> Option<BrainAction> {
        unreachable!()
    }

    fn update(&mut self, _world: &mut World, _brain: Entity) {
        unreachable!()
    }
}

// TODO actually do actions
// Should break into many systems?
fn preform_brain_actions(mut brains: Query<(Entity, &Brain, &mut Transform)>, time: Res<Time>) {
    for (entity, brain, mut transform) in &mut brains {
        // info!("{:?}, {:?}", entity, brain.task.current_action());
        if let Some(task) = brain.task.current_action() {
            match task {
                BrainAction::Idle => {}
                BrainAction::WaitingOnPathfinding => todo!(),
                BrainAction::Walking(target) => {
                    transform.translation = transform.translation
                        + (target - transform.translation.truncate())
                            .normalize()
                            .extend(0.0)
                            * time.delta_seconds()
                            * 3.0
                }
                BrainAction::UsingMachine => todo!(),
                BrainAction::PickupItem => todo!(),
                BrainAction::Build => todo!(),
            }
        }
    }
}
