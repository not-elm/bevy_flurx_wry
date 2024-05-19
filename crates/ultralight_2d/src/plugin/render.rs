use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::prelude::{default, Entity, Image, NonSend, Plugin, Query, ResMut, Update};
use bevy::render::render_resource::Extent3d;
use ul_next::Renderer;

use crate::plugin::UlViewMap;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_renderer,
            render_view
        ));
    }
}

fn update_renderer(
    renderer: NonSend<Renderer>
) {
    renderer.update();
    renderer.render();
}

fn render_view(
    mut images: ResMut<Assets<Image>>,
    view_map: NonSend<UlViewMap>,
    views: Query<(Entity, &Handle<Image>)>,
) {
    for (entity, handle) in views.iter() {
        let Some(mut surface) = view_map
            .0
            .get(&entity)
            .and_then(|view| view.surface())
            else {
                continue;
            };
        if surface.dirty_bounds().is_empty() {
            continue;
        }

        let Some(image) = images.get_mut(handle.id()) else {
            continue;
        };
        let width = surface.width();
        let height = surface.height();
        let Some(pixels) = surface.lock_pixels() else {
            continue;
        };
        image.resize(Extent3d {
            width,
            height,
            ..default()
        });

        image.data = pixels.to_vec();
    }
}
