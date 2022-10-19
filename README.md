# ctr-tablet-rs - Using a Nintendo 3DS as a drawing tablet for PC.

### Note: this is VERY experimental and was created for fun. Follow at your own risk.

### Building (not strictly necessary)

#### Note: I only tested this on my Arch Linux PC. Stuff may break on other OSes.

This is the 3DS side of things. You can build your own version from source or [download the binary release here](https://github.com/adryzz/ctr-tablet-rs/releases/latest)

If you downloaded the binary release you can skip to the next step (Installation)

You'll need

- A rust installation through [rustup](https://rustup.rs/)

- A DevkitPro toolchain installation with [the official instructions](https://devkitpro.org/wiki/Getting_Started)

- A bit of computer and git knowledge

##### Build

- Clone this repository
- Install [cargo-3ds](https://github.com/rust3ds/cargo-3ds) in order to make everything easier to build

(These 2 steps won't be necessary once the issue gets fixed upstream)
- Clone [ctru-rs](https://github.com/rust3ds/ctru-rs), [pthread-3ds](https://github.com/rust3ds/pthread-3ds) and [rust-linker-fix-3ds](https://github.com/rust3ds/rust-linker-fix-3ds).

- Edit the `Cargo.toml` file of each of these packages and point the `ctru-sys` and `ctru-rs` reference of each one to your local reference.

- Download all the 3ds DevkitPro packages through [pacman](https://devkitpro.org/wiki/devkitPro_pacman#Installing_packages)

- Make sure you have your DevkitARM build tools directory in your `PATH` (on linux it should be `/opt/devkitpro/devkitARM/bin`)

- Run `cargo 3ds build --release`

- You should find your built `.3dsx` file in the `target` directory.

### Installing

You'll need:

- A hacked 3DS with easy access to [The Homebrew Launcher](http://smealum.github.io/3ds/) (any model)

#### Install

- Copy the `.3dsx` file you downloaded (or built in the last step) inside the `3ds` directory on your 3DS's SD card. (if it doesn't exist, create it)

- Ensure that your 3DS is connected on the same network as your PC.

- Open The Homebrew Launcher and run the app. You should be greeted with a text-based menu telling you to connectfrom your PC using the IP and port provided.

### PC part

This is the more hacky part, as i am using a [fork of OpenTabletDriver 0.6.0.4](https://github.com/adryzz/OpenTabletDriver/tree/3ds).

You will need:

- [.NET SDK](https://dotnet.microsoft.com/en-us/download/visual-studio-sdks)

- Some very basic reading comprehension

#### Setup

- Clone [my fork of OpenTabletDriver](https://github.com/adryzz/OpenTabletDriver/tree/3ds) making sure that you are on the `3ds` branch

- Go to the file `OpenTabletDriver/Devices/Nintendo3ds/Nintendo3dsInterfaceStream.cs` and change the `IPAddress` line with your 3DS's IP address that you can see on screen. (my 3DS had `10.0.0.5:5000`, so it became `{10, 0, 0, 5}`.

- (Optional) Search on google the dimensions of the lower screen of your particular 3DS variant (the one in the code is an old3ds XL) and replace the values found in `https://github.com/adryzz/OpenTabletDriver/blob/3ds/OpenTabletDriver.Configurations/Configurations/Nintendo/3ds.json`.

- Make sure no other instance of OpenTabletDriver is running

- Go into the `OpenTabletDriver.Daemon` directory and run `dotnet run`

- For the GUI, go into the UX directory for your OS and run `dotnet run`. (Wpf for Windows, MacOS for MacOS and Gtk for Linux).

Everything should be running now!
