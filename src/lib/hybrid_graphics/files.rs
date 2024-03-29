#[cfg(not(debug_assertions))]
pub const BLACKLIST_PATH: &str = "/etc/modprobe.d/blacklist-nvidia.conf";
#[cfg(debug_assertions)]
pub const BLACKLIST_PATH: &str = "./lib_test/etc/modprobe.d/blacklist-nvidia.conf";

pub const BLACKLIST_CONTENT: &str = r#"# Automatically generated by EnvyControl

blacklist nouveau
blacklist nvidia
blacklist nvidia_drm
blacklist nvidia_uvm
blacklist nvidia_modeset
alias nouveau off
alias nvidia off
alias nvidia_drm off
alias nvidia_uvm off
alias nvidia_modeset off
"#;

#[cfg(not(debug_assertions))]
pub const UDEV_INTEGRATED_PATH: &str = "/lib/udev/rules.d/50-remove-nvidia.rules";
#[cfg(debug_assertions)]
pub const UDEV_INTEGRATED_PATH: &str = "./lib_test/lib/udev/rules.d/50-remove-nvidia.rules";

pub const UDEV_INTEGRATED: &str = r#"# Automatically generated by EnvyControl

# Remove NVIDIA USB xHCI Host Controller devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x0c0330", ATTR{power/control}="auto", ATTR{remove}="1"

# Remove NVIDIA USB Type-C UCSI devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x0c8000", ATTR{power/control}="auto", ATTR{remove}="1"

# Remove NVIDIA Audio devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x040300", ATTR{power/control}="auto", ATTR{remove}="1"

# Remove NVIDIA VGA/3D controller devices
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x03[0-9]*", ATTR{power/control}="auto", ATTR{remove}="1"
"#;

#[cfg(not(debug_assertions))]
pub const UDEV_PM_PATH: &str = "/lib/udev/rules.d/80-nvidia-pm.rules";
#[cfg(debug_assertions)]
pub const UDEV_PM_PATH: &str = "./lib_test/lib/udev/rules.d/80-nvidia-pm.rules";

pub const UDEV_PM_CONTENT: &str = r#"# Automatically generated by EnvyControl

# Remove NVIDIA USB xHCI Host Controller devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x0c0330", ATTR{remove}="1"

# Remove NVIDIA USB Type-C UCSI devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x0c8000", ATTR{remove}="1"

# Remove NVIDIA Audio devices, if present
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x040300", ATTR{remove}="1"

# Enable runtime PM for NVIDIA VGA/3D controller devices on driver bind
ACTION=="bind", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x030000", TEST=="power/control", ATTR{power/control}="auto"
ACTION=="bind", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x030200", TEST=="power/control", ATTR{power/control}="auto"

# Disable runtime PM for NVIDIA VGA/3D controller devices on driver unbind
ACTION=="unbind", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x030000", TEST=="power/control", ATTR{power/control}="on"
ACTION=="unbind", SUBSYSTEM=="pci", ATTR{vendor}=="0x10de", ATTR{class}=="0x030200", TEST=="power/control", ATTR{power/control}="on"
"#;

#[cfg(not(debug_assertions))]
pub const XORG_PATH: &str = "/etc/X11/xorg.conf";
#[cfg(debug_assertions)]
pub const XORG_PATH: &str = "./lib_test/etc/X11/xorg.conf";

pub const XORG_INTEL: &str = r#"# Automatically generated by EnvyControl

Section "ServerLayout"
    Identifier "layout"
    Screen 0 "nvidia"
    Inactive "intel"
EndSection

Section "Device"
    Identifier "nvidia"
    Driver "nvidia"
    BusID "[BUS_ID]"
EndSection

Section "Screen"
    Identifier "nvidia"
    Device "nvidia"
    Option "AllowEmptyInitialConfiguration"
EndSection

Section "Device"
    Identifier "intel"
    Driver "modesetting"
EndSection

Section "Screen"
    Identifier "intel"
    Device "intel"
EndSection
"#;

#[cfg(not(debug_assertions))]
pub const MODESET_PATH: &str = "/etc/modprobe.d/nvidia.conf";
#[cfg(debug_assertions)]
pub const MODESET_PATH: &str = "./lib_test/etc/modprobe.d/nvidia.conf";

pub const MODESET_RTD3: &str = r#"# Automatically generated by EnvyControl

options nvidia-drm modeset=1
options nvidia "NVreg_DynamicPowerManagement=0x02"
options nvidia NVreg_UsePageAttributeTable=1 NVreg_InitializeSystemMemoryAllocations=0
"#;

#[cfg(not(debug_assertions))]
pub const SDDM_XSETUP_PATH: &str = "/usr/share/sddm/scripts/Xsetup";
#[cfg(debug_assertions)]
pub const SDDM_XSETUP_PATH: &str = "./lib_test/usr/share/sddm/scripts/Xsetup";

/// empty xsetup config
pub const SDDM_XSETUP_CONTENT: &str = r#"#!/bin/sh
# Xsetup - run as root before the login dialog appears

"#;

pub const NVIDIA_XRANDR_SCRIPT: &str = r#"#!/bin/sh
# Automatically generated by EnvyControl

xrandr --setprovideroutputsource "modesetting" NVIDIA-0
xrandr --auto
"#;

#[cfg(not(debug_assertions))]
pub const NVIDIA_XORG_CONFIG_PATH: &str = "/etc/X11/xorg.conf.d/90-nvidia.conf";
#[cfg(debug_assertions)]
pub const NVIDIA_XORG_CONFIG_PATH: &str = "./lib_test/etc/X11/xorg.conf.d/90-nvidia.conf";

#[cfg(not(debug_assertions))]
pub const SDDM_XSETUP_BACKUP_PATH: &str = "/usr/share/sddm/scripts/Xsetup.backup";
#[cfg(debug_assertions)]
pub const SDDM_XSETUP_BACKUP_PATH: &str = "./lib_test/usr/share/sddm/scripts/Xsetup.backup";
