# QEMU VM available for download

Here is the location of the pre-configured VM used to run the examples presented inside the book.

## VM Location:

https://downloads.embeddedsoftwarewithrustbook.com/vm/xubuntu-rust-distribution.qcow2.zst

## SHA256:

https://downloads.embeddedsoftwarewithrustbook.com/vm/xubuntu-rust-distribution.qcow2.zst.sha256

## Login details

**login**: user 
**password**: EmbeddedRust2026

> Note: You should change the password on first login!

## Installation instructions

On **Linux**, the VM can be run from the Virtual Machine Manager app: https://virt-manager.org/ . This is a GUI based virtual machine manager. Alternatively you can run the `./run-xubuntu.sh` command. This last command assumes that QEMU is installed on your machine.

On **MacOS** the VM can be run using the UTM app: https://mac.getutm.app/

- Install UTM on your MacBook
- After extraction, double click on the `Embedded-Software-with-Rust.utm` file found in this current directory. This will start the UTM application with the provided template (disk-less for now).
- Right click on the *Embedded-Software-with-Rust* entry (left pannel) inside the UTM application
- Select the`Edit` sub-menu
- On the left panel of the `Edit` sub-menu, you will find a `Drives` section. Create a `New` entry
  - I would give the disk a minimum or 32GB but if you can you could aim for 96GB
- Click on the `Import...` button and select the disk file `xubuntu-rust-distribution.qcow2` previously downloaded (you can use a free Mac application called *Keka* (www.keka.io) to extract the `zst` archive).
- Click the `Save` button and you are ready to hit the metaphorical road!

Note (1): There be dragons! I have found that with my (wife's) MacBook Pro M2 thing are fairly slow. The template that I have provided seems to achieve the best result in my case. A better alternative would be for you to run an Aarch64 version of Ubuntu on UTM and follow `../install-guide/embedded-rust-ubuntu-setup.md` instructions to setup your pristine Ubuntu environment. Better still, join the Thinkpad + Linux == <3 fan club. I have been enjoying the ride for many decades, starting with my beloved T22.

Note (2): There be dragons the return! I found that the first time I run the VM on the MacBook Pro, there were some resolution issues (Your mileage may vary...). I could log-in the VM without any problems but past this, I could not click on anything. The solution that I found was to **full screen** the VM window. There I dropped into the Linux console with the `Ctrl+Alt+F2` . Once inside the console, I could force a clean **shutdown**: `shutdown`. Once the VM has properly terminated you can restart it with the UTM GUI. In my case, this fixed all my resolution problems. At this point, inside xubuntu I would suggest you reduce the screen resolution to 1440x900 (16:10) to preserve performances.

On **Windows** the VM can be run using QEMU on windows: www.qemu.org/download/#windows. Once QEMU is installed you can run the script `run-xubuntu.bat`.
