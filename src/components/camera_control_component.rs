use algoe::{bivector::Bivector, rotor::Rotor3};
use gamezap::{ecs::components::transform_component::TransformComponent, new_component};
use nalgebra::{Vector3, Matrix4, Vector4};

new_component!(CameraControlComponent { camera_speed: f32 });

impl CameraControlComponent {
    pub fn new(camera_speed: f32) -> Self {
        Self {
            camera_speed,
            parent: EntityId::MAX,
            id: (EntityId::MAX, TypeId::of::<Self>(), 0),
        }
    }
}

impl ComponentSystem for CameraControlComponent {
    fn update(
        &mut self,
        _device: Arc<Device>,
        _queue: Arc<Queue>,
        component_map: &mut AllComponents,
        engine_details: Rc<Mutex<EngineDetails>>,
        _engine_systems: Rc<Mutex<EngineSystems>>,
        concept_manager: Rc<Mutex<ConceptManager>>,
        _active_camera_id: Option<EntityId>,
        _entities: &mut Vec<Entity>,
        _materials: Option<&mut (Vec<Material>, usize)>,
        _compute_pipelines: &mut [ComputePipeline],
    ) {
        let concept_manager = concept_manager.clone();
        let this_concept_manager = concept_manager.lock().unwrap();
        let details = engine_details.lock().unwrap();
        let scancodes = &details.pressed_scancodes;

        let position = *this_concept_manager
            .get_concept::<Vector3<f32>>(
                (self.id.0, TypeId::of::<TransformComponent>(), self.id.2),
                "position".to_string(),
            )
            .unwrap();

        let matrix = *this_concept_manager
            .get_concept::<Matrix4<f32>>(
                (self.id.0, TypeId::of::<TransformComponent>(), self.id.2),
                "matrix".to_string(),
            )
            .unwrap();
        println!("{matrix}");

        drop(this_concept_manager);

        if scancodes.contains(&sdl2::keyboard::Scancode::Q) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    transform.apply_rotation(
                        concept_manager.clone(),
                        (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                            * std::f32::consts::FRAC_PI_4
                            / 2.0)
                            .exponentiate(),
                    );

                    // transform
                    //     .apply_translation(concept_manager.clone(), -1.0 * position);

                    /* transform.apply_rotation(
                        concept_manager.clone(),
                        (Bivector::new(0.0, 0.0, -1.0) * self.camera_speed).exponentiate(),
                    ); */
                    let rotor = (Bivector::new(0.0, 0.0, 1.0) * self.camera_speed).exponentiate();
                    let mat = Matrix4::from_columns(&[
                        (rotor * Vector3::x_axis().xyz()).to_homogeneous(),
                        (rotor * Vector3::y_axis().xyz()).to_homogeneous(),
                        (rotor * Vector3::z_axis().xyz()).to_homogeneous(),
                        Vector4::new(0.0, 0.0, 0.0, 1.0),
                    ]);


                    // println!("{mat}");

                    transform.apply_rotation(
                        concept_manager.clone(),
                        (Bivector::new(0.0, 0.0, 1.0) * self.camera_speed).exponentiate(),
                    );

                    // transform
                    //     .apply_translation(concept_manager.clone(), position);

                    transform.apply_rotation(
                        concept_manager.clone(),
                        (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                            * -std::f32::consts::FRAC_PI_4
                            / 2.0)
                            .exponentiate(),
                    );
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::E) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    transform.apply_rotation(
                        concept_manager.clone(),
                        (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                            * std::f32::consts::FRAC_PI_4
                            / 2.0)
                            .exponentiate(),
                    );

                    transform
                        .apply_translation(concept_manager.clone(), -1.0 * position);

                    transform.apply_rotation(
                        concept_manager.clone(),
                        (Bivector::new(0.0, 0.0, 1.0) * self.camera_speed).exponentiate(),
                    );

                    transform
                        .apply_translation(concept_manager.clone(), position);


                    transform.apply_rotation(
                        concept_manager.clone(),
                        (Bivector::new(0.0, 0.0, -1.0) * self.camera_speed).exponentiate(),
                    );
                    transform.apply_rotation(
                        concept_manager.clone(),
                        (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                            * -std::f32::consts::FRAC_PI_4
                            / 2.0)
                            .exponentiate(),
                    );
                }
            }
        }
    }
}
