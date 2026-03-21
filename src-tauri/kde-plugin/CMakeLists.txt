cmake_minimum_required(VERSION 3.20)
project(luzumi-desktop-menu)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

# Find KDE Frameworks 6
find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

include(KDEInstallDirs)
include(KDECMakeSettings)

find_package(Qt6 REQUIRED COMPONENTS Core Gui DBus Qml Widgets Svg)
find_package(KF6 REQUIRED COMPONENTS Config CoreAddons Package KirigamiPlatform WindowSystem)

# Fedora uses "Plasma" directly, other distros may use "KF6" component
find_package(Plasma REQUIRED)

# Build the containment action plugin
add_library(plasma_containmentactions_luzumi MODULE
    LuzumiDesktopMenu.cpp
)

target_link_libraries(plasma_containmentactions_luzumi
    Qt6::Core
    Qt6::Gui
    Qt6::DBus
    Qt6::Widgets
    Qt6::Svg
    Plasma::Plasma
)

# Install to KDE plugin directory
install(TARGETS plasma_containmentactions_luzumi
    DESTINATION ${KDE_INSTALL_PLUGINDIR}/plasma/containmentactions
    RENAME com.luzumi.desktopmenu
)

install(FILES metadata.json
    DESTINATION ${KDE_INSTALL_PLUGINDIR}/plasma/containmentactions/
    RENAME com.luzumi.desktopmenu.json
)
