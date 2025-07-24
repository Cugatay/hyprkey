# Hyprkey

The simplest application-aware keymapping tool specifically written for Hyprland

```toml
[firefox]
"CTRL, J" = ", Down"
"CTRL, K" = ", Up"

[Alacritty]
# ...
```

Hyperkey emerged out of a need for an application agnostic keymapper for Hyprland. Currently other available tools in this category are either hard to initialize -dependent on many packages and not ergonomic- or simply don't work/can't detect the current application user focused in to change bindings.

This tool designed with very specific cases: It works and will work only for Hyprland, and it feels like it doesn't work. You don't refresh or restart hyprkey when you update your configurations.

## Main Features
- **Application aware keymapping:** This is something that's not available in Hyprland by default. If you bind a key for an app, that keybinding is not available anymore in your system. Hyprkey enables you to define keymappings fearlessly! When you define a keybinding, you can continue using it in any other app without any affect.
- **Very specific user-base:** It doesn't try to be an every system's app, as in many keymapper tools tried it and they fail in experience for Hyprland.
- **Designed to be forgotton:** I know, this sounds copy-writy, but just run hyprkey with `exec-once` in *hyprland.conf*, then never run the command again. It watches your config file for any change and automatically gets updated when any change applied. Hyprkey command on terminal doesn't take any parameter, just change the config file. Just as promised: Never run hyprkey command again, forget it.
- **No dependency:** Installing Hyprkey on your system is enough to run it. At the moment we use `cargo` for installation, but other than that, you're good to go once the `hyprkey` command is available in your terminal
- **Blazingly f...:** Used Rust, and you know the rest.

## Getting Started

#### 1. Install the executable:
```bash
cargo install --git https://github.com/Cugatay/hyprkey hyprkey
```

#### 2. Add it to your `hyprland.conf`:
```
# in ~/.config/hypr/hyprland.conf:
exec-once = hyprkey
```

At this point, hyprkey will run automatically next time you start your system. Until then, you can simply start it by typing your terminal `hyprkey` command.

#### 3. Configuration
Configuration file for hyprkey must be located in *~/.config/hyprkey/config.toml*, create a file at that location if you don't have it.

Configuration structure is simple:

```toml
[application-class]
"from, key": "to, key"
```

- **application-class:** You can learn the class of your application by typing `hyprctl clients`, then search for the app you want to create keybindings for, find its class.
- **from, key:** The key you want to remap from. In the top of this README, there was an example for Firefox app, where we bind Ctrl + J to Down key. You can use it as an example. Don't forget that the names of keys and structuring them are the same in Hyprland's configuration file. Look for the 4. step for important note
- **to, key:** The key you want to remap to. This key will be pressed when you press to the **from, key**. Again, follows the same structure and key names as Hyprland's config files.

#### 4. Important Note
As you can see in the Firefox example in the beginning of this README, if you want to remap (both from and to keys) just one key (Down key on Firefox example) you'll write the following:

```toml
[firefox]
"CTRL, J" = "Down"
```

The following is the right way:

```toml
[firefox]
"CTRL, J" = ", Down" # Uses comma before if you have only one key
```

This is because Hyprland's key binding command uses this same structure. In the next updates we'll simply this by letting you only write the key name.


## Status
This is a very early stage for Hyprkey, it has many known and probably unknown bugs. Still, as the maintainer of it I started using it daily, as there was no other alternative and I needed it desperately. So, I have to improve it, otherwise I'll be missing the dream dev environment I always wanted.

In this state, Hyprkey project and I would appreciate any help coming from you; including word of mouth or coding. Thank you all in advance for your patience and support. If you want to contribute in this project, contribution guide is under construction at the moment, so don't be shy to get creative on your commit messages :)

## Anything Else In Mind?

Just open a PR, and ask any question, share any thought. I'll be answering it whenever I touch my keyboard.
