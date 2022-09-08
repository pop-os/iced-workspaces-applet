# Iced Workspaces Applet

Proof of concept applet for Cosmic.

## Building the project

Gnome Builder works well for building the project as a flatpak during development. (It may be helpful to also install rust-analyzer and add it to [PATH](https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary).)

### Builing a deb
`dpkg-buildpackage -b -d`

### Flatpak CLI
Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak install org.gnome.Sdk//42 org.freedesktop.Sdk.Extension.rust-stable//21.08 org.gnome.Platform//42
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

Once the project is build, run the command below. Replace Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```

## Community

Join the  Pop_OS!, GNOME, and gtk-rs communities!
- [Pop_OS! Mattermost](https://chat.pop-os.org/)
- [Matrix chat](https://matrix.to/#/#rust:gnome.org): chat with other developers using gtk-rs
- [Discourse forum](https://discourse.gnome.org/tag/rust): topics tagged with `rust` on the GNOME forum.
- [GNOME circle](https://circle.gnome.org/): take inspiration from applications and libraries already extending the GNOME ecosystem.

## Credits

- [Gtk Rust Template](https://gitlab.gnome.org/World/Rust/gtk-rust-template/-/tree/master/) is what this project is directly built on top of.
- [Podcasts](https://gitlab.gnome.org/World/podcasts)
- [Shortwave](https://gitlab.gnome.org/World/Shortwave)
