use amethyst::{
    ecs::{System, Read, ReadStorage, WriteStorage},
    core::timing::Time,
    core::Transform,
    prelude::Entities,
};
use crate::components::{Ball, Block, Paddle, Velocity};

// NOTE: I recommend splitting this collision into at least four separate systems.
//
//  1. cross( paddle+position, ball+position+velocity )
//      ball bounces at an angle according to where it hits horizontally on the paddle
//
//  2. cross( block+position, ball+position+velocity )
//      ball bounces at an angle according to its incoming angle
//
//  3. cross( block+position, ball+position+status )
//      a "struck" component is added to the block and notes any power-ups the ball has
//
//  4. block+struck
//      a block evaluates whether it needs to die based on the damage caused by "struck"
//
//  The goal is for systems to be as simple as possible. Think of them as SQL queries that require
//  regular maintenance. You don't want any one system to serve distinct purposes.
//
//  (I'm using an ad-hoc syntax above to describe "queries" against the ecs... The syntax is:
//      - a+b means "entities having all of an `a` and a `b` component"
//      - a+b+c means "entities having all of an `a`, a `b`, and a `c` component"
//      - cross(x, y) means "the cross product of entities in x and entities in y"
//  )

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    // TODO too many entries in this tuple
    type SystemData = (
        entities: Entities<'a>,
        balls: ReadStorage<'a, Ball>,
        blocks: ReadStorage<'a, Block>,
        paddles: ReadStorage<'a, Paddle>,
        transform: ReadStorage<'a, Transform>,
        velocity: WriteStorage<'a, Velocity>,
        time: Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}
