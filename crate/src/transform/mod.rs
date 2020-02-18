
mod matrix;
mod vec3;
mod quat;
mod values;
mod components;

pub use self::matrix::*;
pub use self::vec3::*;
pub use self::quat::*;
pub use self::values::*;
pub use self::components::*;

use shipyard::prelude::*;
use crate::hierarchy::*;

/*
    these need access to the whole hierarchy
*/
pub trait TransformHierarchyMut {
    fn spawn_child(&mut self, parent: EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) -> EntityId;
}

pub type TransformHierarchyStoragesMut<'a, 'b> = (
    &'b mut EntitiesViewMut<'a>, 
    &'b mut ViewMut<'a, Parent>, 
    &'b mut ViewMut<'a, Child>,
    &'b mut ViewMut<'a, Translation>,
    &'b mut ViewMut<'a, Rotation>,
    &'b mut ViewMut<'a, Scale>,
    &'b mut ViewMut<'a, LocalTransform>,
    &'b mut ViewMut<'a, WorldTransform>,
);

impl TransformHierarchyMut for TransformHierarchyStoragesMut<'_, '_> {
    fn spawn_child(&mut self, parent: EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) -> EntityId {

        let (
            entities, 
            parents,
            childs,
            translations,
            rotations,
            scales,
            local_transforms,
            world_transforms
        ) = self;

        let translation = translation.unwrap_or_default();
        let rotation = rotation.unwrap_or_default();
        let scale = scale.unwrap_or(Vec3::new(1.0, 1.0, 1.0));
        let local_matrix = Matrix4::default(); //Matrix4::new_from_trs(&translation, &rotation, &scale);
        let world_matrix = Matrix4::default();

        let entity = entities.add_entity( 
                (
                    &mut **translations,
                    &mut **rotations,
                    &mut **scales,
                    &mut **local_transforms,
                    &mut **world_transforms
                ),
                (
                    Translation(translation),
                    Rotation(rotation),
                    Scale(scale),
                    LocalTransform(local_matrix),
                    WorldTransform(world_matrix)
                )
        );

        {

            (&mut **entities, &mut **parents, &mut **childs).attach(entity, parent);
        }
        entity
    }

}


// these methods don't need access to the hierarchy
pub trait TransformMut {
    fn set_trs(&mut self, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>);
}

pub type TransformStoragesMut<'a, 'b> = (
    &'b mut EntitiesViewMut<'a>, 
    &'b mut ViewMut<'a, Translation>,
    &'b mut ViewMut<'a, Rotation>,
    &'b mut ViewMut<'a, Scale>,
    &'b mut ViewMut<'a, LocalTransform>,
    &'b mut ViewMut<'a, WorldTransform>,
);

impl TransformMut for TransformStoragesMut<'_, '_> {
    fn set_trs(&mut self, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) {
        let (
            entities, 
            translations,
            rotations,
            scales,
            local_transforms,
            world_transforms
        ) = self;

        if let Some((t,r,s)) = (&mut **translations, &mut **rotations, &mut **scales).get(entity).iter_mut().next() {
            if let Some(translation) = translation {
                t.0.copy_from(&translation);
            }
            if let Some(rotation) = rotation {
                r.0.copy_from(&rotation);
            }
            if let Some(scale) = scale {
                s.0.copy_from(&scale);
            }
        }
    }
}