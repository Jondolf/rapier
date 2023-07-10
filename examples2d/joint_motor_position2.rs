use rapier2d::prelude::*;
use rapier_testbed2d::Testbed;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    /*
     * The ground
     */
    let ground_size = 5.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, -ground_height]);
    let ground_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height);
    colliders.insert_with_parent(collider, ground_handle, &mut bodies);

    /*
     * A rectangle on a motor with target position.
     */
    for num in 0..9 {
        let x_pos = -6.0 + 1.5 * num as f32;
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x_pos, 2.0])
            .rotation(std::f32::consts::PI)
            .can_sleep(false);
        let handle = bodies.insert(rigid_body);
        let collider = ColliderBuilder::cuboid(0.1, 0.5);
        colliders.insert_with_parent(collider, handle, &mut bodies);

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![x_pos, 1.5])
            .local_anchor2(point![0.0, -0.5])
            .motor_position(
                (std::f32::consts::PI - std::f32::consts::PI / 4.0 * num as f32),
                1000.0,
                150.0,
            );
        impulse_joints.insert(ground_handle, handle, joint, true);
    }

    /*
     * A rectangle on a motor with limits.
     */
    for num in 2..3 {
        let x_pos = -6.0 + 1.5 * num as f32;
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x_pos, 5.0])
            .angvel(4.0)
            .can_sleep(false);
        let handle = bodies.insert(rigid_body);
        let collider = ColliderBuilder::cuboid(0.1, 0.5);
        colliders.insert_with_parent(collider, handle, &mut bodies);

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![x_pos, 5.0])
            .local_anchor2(point![0.0, -0.5])
            .motor_velocity(1.5, 30.0)
            .motor_max_force(100.0)
            .limits([
                -std::f32::consts::PI,
                std::f32::consts::PI / 4.0 * num as f32,
            ]);
        impulse_joints.insert(ground_handle, handle, joint, true);
    }

    /*
     * Set up the testbed.
     */
    testbed.set_world_with_params(
        bodies,
        colliders,
        impulse_joints,
        multibody_joints,
        vector![0.0, 0.0],
        (),
    );
    testbed.look_at(point![0.0, 0.0], 40.0);
}
