use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, Handle};
use bevy::prelude::{Added, Commands, Entity, Image, NonSend, NonSendMut, Query, ResMut};
use ul_next::{Renderer, View};
use ul_next::view::ViewConfig;

use crate::bundle::{Load, ViewSize};
use crate::plugin::{DownKey, UlViewMap};

pub struct SetupViewPlugin;

impl Plugin for SetupViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_view);
    }
}

fn setup_view(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut view_map: NonSendMut<UlViewMap>,
    mut views: Query<(Entity, &mut Handle<Image>, &ViewSize, &Load), Added<Load>>,
    renderer: NonSend<Renderer>,
) {
    for (entity, mut handle, size, load) in views.iter_mut() {
        if let Some(view) = renderer.create_view(size.x, size.y, &ViewConfig::start()
            .is_accelerated(false)
            .enable_images(true)
            .enable_javascript(true)
            .build()
            .unwrap(), None,
        ) {
            commands.entity(entity).insert(DownKey::default());
            *handle = images.add(Image::default());
            attach_script(&view, &entity);
            load_view(&view, load);
            view_map.0.insert(entity, view);
        }
    }
}

#[inline]
fn attach_script(view: &View, entity: &Entity) {
    view.evaluate_script(&format!("const entity = {}", serde_json::to_string(entity).unwrap()))
        .unwrap()
        .unwrap();
    view.evaluate_script(include_str!("initialize.js")).unwrap().unwrap();
}

#[inline]
fn load_view(view: &View, load: &Load) {
    match load {
        Load::Html(html) => {
            view.load_html(html).unwrap();
        }
        Load::Uri(uri) => {
            view.load_url(uri).unwrap();
        }
    }
}
