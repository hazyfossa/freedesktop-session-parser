use envy::define_env;

// https://www.freedesktop.org/software/systemd/man/latest/pam_systemd.html#type=
define_env!(SessionKind = "XDG_SESSION_TYPE");
crate::strenum!(pub SessionKind {
    Unspecified = "unspecified",
    TTY = "tty",
    X11 = "x11",
    Wayland = "wayland",
    Mir = "mir",
    Web = "web",
});

define_env!(pub Desktop(String) = "XDG_SESSION_DESKTOP");
define_env!(pub DesktopList(String) = "XDG_CURRENT_DESKTOP");

impl DesktopList {
    pub fn to_vec(self) -> Vec<String> {
        self.split(";").map(String::from).collect()
    }
}
