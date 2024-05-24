use std::sync::{Arc, Mutex};

use bevy::app::{App, Update};
use bevy::prelude::{Commands, Entity, Plugin, Query, Res, Resource};
use bevy_flurx::prelude::Reactor;
use wry::PageLoadEvent;

use crate::core::bundle::OnPageLoad;

pub(crate) struct OnPageArgs {
    pub event: PageLoadEvent,
    pub uri: String,
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub(crate) struct PageLoadEventQueue(pub Arc<Mutex<Vec<OnPageArgs>>>);

pub struct OnPageLoadPlugin;

impl Plugin for OnPageLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PageLoadEventQueue>()
            .add_systems(Update, on_page);
    }
}

fn on_page(
    mut commands: Commands,
    queue: Res<PageLoadEventQueue>,
    views: Query<&OnPageLoad>,
) {
    for args in std::mem::take(&mut *queue.0.lock().unwrap()) {
        if let Some(f) = views
            .get(args.entity)
            .ok()
            .and_then(|on_page| on_page.0.as_ref())
        {
            let seed = f(args.event, args.uri);
            commands.spawn(Reactor::schedule(|task| async move {
                task.will(Update, seed).await;
            }));
        }
    }
}