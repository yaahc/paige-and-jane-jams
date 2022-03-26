use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::Vec2;
use rapier2d::prelude::*;

// actually just rapier2d
struct Reality {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
}

impl Default for Reality {
    fn default() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }
}

impl Reality {
    fn update(&mut self) {
        let gravity = vector![0.0, -9.81];
        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }
}

struct Ball<'a>(&'a RigidBody);

impl Ball<'_> {
    fn point(&self) -> Vec2 {
        Vec2::new(self.0.translation().x, self.0.translation().y)
    }
}

struct TransPlatformer {
    reality: Reality,
    ball: RigidBodyHandle,
}

impl TransPlatformer {
    #[must_use]
    fn new(ctx: &Context) -> Self {
        let mut reality = Reality::default();
        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        reality.collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![100.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = reality.rigid_body_set.insert(rigid_body);
        reality.collider_set.insert_with_parent(
            collider,
            ball_body_handle,
            &mut reality.rigid_body_set,
        );

        Self {
            reality,
            ball: ball_body_handle,
        }
    }

    fn ball(&self) -> Ball<'_> {
        Ball(&self.reality.rigid_body_set[self.ball])
    }
}

impl event::EventHandler<ggez::GameError> for TransPlatformer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.reality.update();
        let ball = self.ball();
        println!("Ball altitude: {}", ball.0.translation().y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let origin = Vec2::new(0.0, 0.0);
        let body = graphics::MeshBuilder::new()
            .circle(graphics::DrawMode::fill(), origin, 20.0, 2.0, Color::WHITE)?
            .build(ctx)?;

        graphics::draw(ctx, &body, (self.ball().point(),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("trans platformer", "paige & jane");
    let (ctx, event_loop) = cb.build()?;
    let state = TransPlatformer::new(&ctx);
    event::run(ctx, event_loop, state)
}
