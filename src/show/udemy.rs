use crate::Downloadable;
use rand;
use std::fmt::Display;

#[derive(Debug)]
pub enum Udemy {
    Video {
        name: Option<String>,
        url: String,
        path: Option<String>,
    },
    Recourse {
        name: Option<String>,
        url: String,
        path: Option<String>,
    },
}

static mut COUNTER: u32 = 0;

impl Default for Udemy {
    fn default() -> Self {
        let counter = unsafe {
            COUNTER += 1;
            COUNTER
        };

        match rand::random::<bool>() {
            true => Udemy::Video {
                name: Some(format!("Video {}", counter)),
                url: format!("https://www.udemy.com/video/{}", counter),
                path: None,
            },
            false => Udemy::Recourse {
                name: Some(format!("Recourse {}", counter)),
                url: format!("https://www.udemy.com/recourse/{}", counter),
                path: None,
            },
        }
    }
}

impl Display for Udemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Udemy::Video { name, .. } => name,
            Udemy::Recourse { name, .. } => name,
        };

        write!(f, "{}", name.as_ref().unwrap_or(&"Unknown".to_string()))
    }
}

impl Downloadable for Udemy {}

impl Udemy {
    pub fn new_video(
        name: impl Into<String>,
        url: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Udemy::Video {
            name: Some(name.into()),
            url: url.into(),
            path: Some(path.into()),
        }
    }

    pub fn new_recourse(
        name: impl Into<String>,
        url: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Udemy::Recourse {
            name: Some(name.into()),
            url: url.into(),
            path: Some(path.into()),
        }
    }

    pub fn video_name(name: impl Into<String>) -> Self {
        let name: String = name.into();

        Udemy::Video {
            name: Some(name.clone()),
            url: format!("https://www.udemy.com/video/{}", name),
            path: None,
        }
    }
}
