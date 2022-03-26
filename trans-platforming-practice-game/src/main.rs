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

const WORLD_WIDTH: f32 = 100.0;
const WORLD_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 8.0;

impl TransPlatformer {
    #[must_use]
    fn new(ctx: &Context) -> Self {
        let mut reality = Reality::default();
        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(WORLD_WIDTH, 0.1).build();
        reality.collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![WORLD_WIDTH / 2.0, WORLD_HEIGHT])
            .build();
        let collider = ColliderBuilder::ball(BALL_RADIUS).restitution(0.95).build();
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

fn world_to_screen(ctx: &Context, v: f32) -> f32 {
    let (screen_width, _) = graphics::size(ctx);
    v * screen_width / WORLD_WIDTH
}

fn world_to_screen_pos(ctx: &Context, world_point: Vec2) -> Vec2 {
    let (screen_width, screen_height) = graphics::size(ctx);
    Vec2::new(
        world_to_screen(ctx, world_point.x),
        screen_height - world_to_screen(ctx, world_point.y))
}

fn world_to_screen_uhh(ctx: &Context, world_point: Vec2) -> Vec2 {
    let (screen_width, screen_height) = graphics::size(ctx);
    Vec2::new(
        world_to_screen(ctx, world_point.x),
        -world_to_screen(ctx, world_point.y))
}

impl event::EventHandler<ggez::GameError> for TransPlatformer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.reality.update();
        let ball = self.ball();
        println!("Ball altitude: {}", ball.0.translation().y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("screen size: {:?}", graphics::size(ctx));
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());
        let size = graphics::size(ctx);
        graphics::set_screen_coordinates(ctx, [0.0, 0.0, size.0, size.1].into())?;
        let bg_pos = world_to_screen_pos(ctx, Vec2::new(4.0, 4.0));
        let bg_size = world_to_screen_uhh(ctx,
            Vec2::new(WORLD_WIDTH - 8.0, WORLD_HEIGHT - 8.0));
        let bg_rect = graphics::Rect::new(
            bg_pos.x, bg_pos.y, bg_size.x, bg_size.y);
        dbg!(&bg_rect);

        let bg = graphics::Mesh::new_rectangle(ctx,
            graphics::DrawMode::Fill(Default::default()),
            bg_rect,
            [0.1, 0.2, 0.3, 1.0].into())?;
        graphics::draw(ctx, &bg, (Vec2::new(0.0, 0.0),))?;

        for i in 0..10 {
            for j in 0..10 {
                let c = graphics::Mesh::new_circle(ctx,
                    graphics::DrawMode::Fill(Default::default()),
                    Vec2::new(i as f32 * 50.0, j as f32 * 50.0),
                    5.0,
                    1.0,
                    [1.0, j as f32 * 0.1, 0.0, 1.0].into())?;
                graphics::draw(ctx, &c, (Vec2::new(5.0, 5.0),))?;
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                let c = graphics::Mesh::new_circle(ctx,
                    graphics::DrawMode::Fill(Default::default()),
                    Vec2::new(0.0, 0.0),
                    5.0,
                    1.0,
                    [0.0, j as f32 * 0.1, 1.0, 1.0].into())?;
                graphics::draw(ctx, &c,
                    (world_to_screen_pos(ctx, Vec2::new(i as f32 * 10.0, j as f32 * 10.0)),))?;
            }
        }

        let radius = BALL_RADIUS * graphics::size(ctx).0 / WORLD_WIDTH;
        let origin = Vec2::new(0.0, 0.0);
        let body = graphics::MeshBuilder::new()
            .circle(graphics::DrawMode::fill(), origin, radius, 0.4, Color::WHITE)?
            .build(ctx)?;

        let screen_point = world_to_screen_pos(ctx, self.ball().point());
        graphics::draw(ctx, &body, (screen_point,))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("trans platformer", "paige & jane");
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_mode(&mut ctx, ggez::conf::WindowMode {
        resizable: true,
        .. Default::default()
    }).expect(">:U");
    let state = TransPlatformer::new(&ctx);
    event::run(ctx, event_loop, state)
}
