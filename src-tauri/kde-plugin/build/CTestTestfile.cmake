# CMake generated Testfile for 
# Source directory: /home/kujau/Desktop/projects/luzumi/src-tauri/kde-plugin
# Build directory: /home/kujau/Desktop/projects/luzumi/src-tauri/kde-plugin/build
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test([=[appstreamtest]=] "/usr/bin/cmake" "-DAPPSTREAMCLI=/usr/bin/appstreamcli" "-DINSTALL_FILES=/home/kujau/Desktop/projects/luzumi/src-tauri/kde-plugin/build/install_manifest.txt" "-P" "/usr/share/ECM/kde-modules/appstreamtest.cmake")
set_tests_properties([=[appstreamtest]=] PROPERTIES  _BACKTRACE_TRIPLES "/usr/share/ECM/kde-modules/KDECMakeSettings.cmake;173;add_test;/usr/share/ECM/kde-modules/KDECMakeSettings.cmake;191;appstreamtest;/usr/share/ECM/kde-modules/KDECMakeSettings.cmake;0;;/home/kujau/Desktop/projects/luzumi/src-tauri/kde-plugin/CMakeLists.txt;13;include;/home/kujau/Desktop/projects/luzumi/src-tauri/kde-plugin/CMakeLists.txt;0;")
