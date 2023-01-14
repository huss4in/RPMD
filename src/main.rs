#![allow(dead_code, unused_variables)]

use rpmd::prelude::*;

fn main() {
    let config = rpmd::Config {
        basedir: std::path::Path::new("~/Downloads").into(),
        cores: num_cpus::get(),
    };

    let create_episode = || {
        (1..=10)
            .map(|_| rpmd::show::Udemy::default())
            .collect::<Vec<_>>()
    };

    let show = rpmd::Show {
        name: "Show 1".into(),
        seasons: vec![
            rpmd::Season {
                name: "Season 1".into(),
                episodes: create_episode(),
            },
            rpmd::Season {
                name: "Season 2".into(),
                episodes: create_episode(),
            },
        ],
    };

    let thread_pool = rpmd::thread::ThreadPool::new(config.cores);

    for season in show.seasons {
        for episode in season.episodes {
            std::thread::sleep(std::time::Duration::from_millis(10));
            thread_pool.execute(|i| episode.download(Some(i)));
        }
    }
}
