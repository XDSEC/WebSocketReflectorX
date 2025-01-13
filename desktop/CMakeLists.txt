cmake_minimum_required(VERSION 3.20)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

string(TIMESTAMP BUILD_TIME UTC)

set(IN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/variables.h.in")
set(GENERATED_FILE_PATH "${CMAKE_BINARY_DIR}/GeneratedFiles")
set(OUT_FILE "${GENERATED_FILE_PATH}/variables.h")
configure_file("${IN_FILE}" "${OUT_FILE}" @ONLY)

include_directories(${GENERATED_FILE_PATH})

add_subdirectory(singleapplication)

set(
  SOURCES
    # Entrypoint
    main.cc
    # Modules
    log.cc log.h
    pool.cc pool.h
    cors.cc cors.h
    daemon.cc daemon.h
    ui.cc ui.h
    # Resources
    components/components.qrc
    ui/ui.qrc resources/icon.rc
)

if (APPLE)
    # And the following tells CMake where to find and install the file itself.
    set(APPLE_ICON "${CMAKE_SOURCE_DIR}/macos/WebSocketReflectorX.icns")
    set_source_files_properties(${APPLE_ICON} PROPERTIES
           MACOSX_PACKAGE_LOCATION "Resources")
endif()

qt_add_big_resources(RESOURCES resources/i18n.qrc resources/resources.qrc)

if(WIN32)
  add_executable(${PROJECT_NAME}-desktop WIN32 ${SOURCES} ${RESOURCES})
elseif(APPLE)
  set(CMAKE_XCODE_ATTRIBUTE_PRODUCT_BUNDLE_IDENTIFIER "tech.woooo.wsrx")
  set(CMAKE_XCODE_ATTRIBUTE_MARKETING_VERSION "${FULL_VERSION}")
  # set(CMAKE_OSX_ARCHITECTURES "x86_64;arm64")
  set(CMAKE_OSX_DEPLOYMENT_TARGET "12.0")
  add_executable(${PROJECT_NAME}-desktop MACOSX_BUNDLE ${SOURCES} ${RESOURCES} ${APPLE_ICON})
else()
  add_executable(${PROJECT_NAME}-desktop ${SOURCES} ${RESOURCES})
endif()

target_link_libraries(
  ${PROJECT_NAME}-desktop
  ${QT_LIBRARIES} 
  SingleApplication::SingleApplication
)

set_target_properties(
  ${PROJECT_NAME}-desktop PROPERTIES
    AUTOMOC ON
    AUTORCC ON
    AUTOUIC ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

if (APPLE)
  set_target_properties(
    ${PROJECT_NAME}-desktop PROPERTIES
    MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/macos/Info.plist.in
  )
  set(MACOSX_BUNDLE_DISPLAY_NAME "WebSocket Reflector X")
  set(MACOSX_BUNDLE_GUI_IDENTIFIER "tech.woooo.wsrx")
  set(MACOSX_BUNDLE_BUNDLE_NAME "wsrx")
  set(MACOSX_BUNDLE_ICON_FILE WebSocketReflectorX.icns)
  set(MACOSX_BUNDLE_INFO_STRING "WebSocket Reflector X - Controlled WebSocket-TCP tunnel.")
  set(MACOSX_BUNDLE_LONG_VERSION_STRING "wsrx ${FULL_VERSION}")
  set(MACOSX_BUNDLE_SHORT_VERSION_STRING "${VERSION_MAJOR}.${VERSION_MINOR}")
  set(MACOSX_BUNDLE_BUNDLE_VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")
  set(MACOSX_BUNDLE_COPYRIGHT "2021-2024 XDSEC Developers")
endif()

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
  MANIFEST_PATH ../Cargo.toml
  FEATURES "binary"
)

set_target_properties(
  ${PROJECT_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)
