use gtk4::gio;
use gtk4::prelude::*;
use libadwaita as adw;

use crate::window::WallrusWindow;

pub struct WallrusApplication {
    app: adw::Application,
}

impl WallrusApplication {
    pub fn new(app_id: &str) -> Self {
        let app = adw::Application::builder()
            .application_id(app_id)
            .flags(gio::ApplicationFlags::FLAGS_NONE)
            .build();

        app.connect_startup(|_| {
            // Use AdwStyleManager to follow the system color scheme.
            // This is the correct replacement for the deprecated
            // gtk-application-prefer-dark-theme GtkSettings flag.
            let style = adw::StyleManager::default();
            style.set_color_scheme(adw::ColorScheme::Default);
        });

        app.connect_activate(Self::on_activate);

        Self { app }
    }

    fn on_activate(app: &adw::Application) {
        let window = WallrusWindow::new(app);
        window.present();
    }

    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}
