cmake_minimum_required(VERSION 3.20)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(IN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/variables.h.in")
set(GENERATED_FILE_PATH "${CMAKE_BINARY_DIR}/GeneratedFiles")
set(OUT_FILE "${GENERATED_FILE_PATH}/variables.h")
configure_file("${IN_FILE}" "${OUT_FILE}" @ONLY)

include_directories(${GENERATED_FILE_PATH})

set(
  SOURCES
    # Entrypoint
    main.cc
    # Modules
    log.cc log.h
    origin.cc origin.h
    pool.cc pool.h
    daemon.cc daemon.h
    ui.cc ui.h
    # Resources
    components/components.qrc
    resources/i18n.qrc
    resources/resources.qrc
    ui/ui.qrc
)

add_executable(${PROJECT_NAME}-desktop ${SOURCES})

target_link_libraries(${PROJECT_NAME}-desktop ${QT_LIBRARIES})

set_target_properties(
  ${PROJECT_NAME}-desktop PROPERTIES
    AUTOMOC ON
    AUTORCC ON
    AUTOUIC ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

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
  wsrx PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)
