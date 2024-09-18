use algoe::{bivector::Bivector, rotor::Rotor3};
use gamezap::{ecs::components::transform_component::TransformComponent, new_component};
use nalgebra::Vector3;

new_component!(CameraControlComponent {
    camera_speed: f32,
    vertical_rotation: Rotor3
});

impl CameraControlComponent {
    pub fn new(camera_speed: f32) -> Self {
        Self {
            camera_speed,
            parent: EntityId::MAX,
            vertical_rotation: (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                * -std::f32::consts::FRAC_PI_4
                / 3.0)
                .exponentiate(),
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

        let mut position = *this_concept_manager
            .get_concept::<Vector3<f32>>(
                (self.id.0, TypeId::of::<TransformComponent>(), self.id.2),
                "position".to_string(),
            )
            .unwrap();
        drop(this_concept_manager);

        if scancodes.contains(&sdl2::keyboard::Scancode::Q) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    transform.apply_rotation(
                        concept_manager.clone(),
                        Rotor3::new(
                            self.vertical_rotation.scalar,
                            self.vertical_rotation.bivector * -1.0,
                        ),
                    );

                    let rotor = (Bivector::new(0.0, 0.0, 1.0) * self.camera_speed).exponentiate();

                    let new_position = rotor * position;

                    transform.apply_rotation(concept_manager.clone(), rotor);

                    transform.apply_translation(concept_manager.clone(), new_position - position);

                    position = new_position;

                    transform.apply_rotation(concept_manager.clone(), self.vertical_rotation);
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::E) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    transform.apply_rotation(
                        concept_manager.clone(),
                        Rotor3::new(
                            self.vertical_rotation.scalar,
                            self.vertical_rotation.bivector * -1.0,
                        ),
                    );

                    let rotor = (Bivector::new(0.0, 0.0, -1.0) * self.camera_speed).exponentiate();

                    let new_position = rotor * position;

                    transform.apply_rotation(concept_manager.clone(), rotor);

                    transform.apply_translation(concept_manager.clone(), new_position - position);

                    position = new_position;

                    transform.apply_rotation(concept_manager.clone(), self.vertical_rotation);
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::W) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let rotation = (algoe::bivector::Bivector::new(0.0, 1.0, 0.0)
                        * self.camera_speed)
                        .exponentiate();
                    transform.apply_rotation(concept_manager.clone(), rotation);

                    self.vertical_rotation = self.vertical_rotation * rotation;
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::S) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let rotation = (algoe::bivector::Bivector::new(0.0, -1.0, 0.0)
                        * self.camera_speed)
                        .exponentiate();
                    transform.apply_rotation(concept_manager.clone(), rotation);

                    self.vertical_rotation = self.vertical_rotation * rotation;
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::A) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let new_position = position * (1.0 - self.camera_speed);
                    transform.apply_translation(concept_manager.clone(), new_position - position);

                    position = new_position;
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::D) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let new_position = position * (1.0 + self.camera_speed);
                    transform.apply_translation(concept_manager.clone(), new_position - position);

                    position = new_position;
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::Space) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let new_position = position + Vector3::new(0.0, self.camera_speed, 0.0);
                    transform.apply_translation(concept_manager.clone(), new_position - position);
                    position = new_position;
                }
            }
        }

        if scancodes.contains(&sdl2::keyboard::Scancode::LCtrl) {
            for comp in component_map.get_mut(&self.parent).unwrap() {
                if let Some(transform) = comp.as_any_mut().downcast_mut::<TransformComponent>() {
                    let new_position = position - Vector3::new(0.0, self.camera_speed, 0.0);
                    transform.apply_translation(concept_manager.clone(), new_position - position);
                    position = new_position;
                }
            }
        }
    }
}
