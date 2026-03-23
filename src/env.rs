use envy::define_env;

// https://www.freedesktop.org/software/systemd/man/latest/pam_systemd.html#type=
define_env!(SessionKind = "XDG_SESSION_TYPE");
crate::strenum!(pub SessionKind {
    Unspecified,
    TTY,
    X11,
    Wayland,
    Mir,
    Web,
});

define_env!(pub Desktop(String) = "XDG_SESSION_DESKTOP");
define_env!(pub DesktopList(String) = "XDG_CURRENT_DESKTOP");

impl DesktopList {
    pub fn to_vec(self) -> Vec<String> {
        self.split(";").map(String::from).collect()
    }
}
