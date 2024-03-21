cmake_minimum_required(VERSION 3.20)

set(VERSION_MAJOR 0)
set(VERSION_MINOR 2)
set(VERSION_PATCH 0)

execute_process(
    COMMAND git describe --always --dirty
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    OUTPUT_VARIABLE GIT_VERSION
    OUTPUT_STRIP_TRAILING_WHITESPACE
)
set(FULL_VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}.g${GIT_VERSION}")
message(STATUS "Current build version: ${FULL_VERSION}")

project(
  wsrx
    LANGUAGES C CXX
    VERSION 0.2.0
)

set(
  CMAKE_MODULE_PATH
    ${CMAKE_MODULE_PATH}
    ${ECM_MODULE_PATH}
)
include(GenerateExportHeader)
include(CMakePackageConfigHelpers)
include(FeatureSummary)
include(WriteBasicConfigVersionFile)
include(CheckFunctionExists)

set(CMAKE_CXX_STANDARD 17)
set(CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_MAKE_PROGRAM ninja)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall --std=c++17 -O3 -fPIC" )
set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -Wall -O3 -fPIC" )
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

if(MSVC)
  set(CMAKE_CXX_FLAGS "/source-charset:utf-8 /execution-charset:utf-8")
endif()

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(QT_MIN_VERSION 6.5.0)
find_package(Qt6 ${QT_MIN_VERSION} REQUIRED COMPONENTS Core Quick Gui QuickControls2 Widgets Network Concurrent WebSockets Core5Compat)
set(QT_LIBRARIES Qt6::Core Qt6::Widgets Qt6::Network Qt6::Quick Qt6::Gui Qt6::QuickControls2 Qt6::Concurrent Qt6::WebSockets Qt6::Core5Compat)

if(APPLE)
    find_package(Qt6 REQUIRED COMPONENTS MacExtras)
    list(APPEND QT_LIBRARIES Qt6::MacExtras)
elseif(WIN32)
    find_package(Qt6 REQUIRED COMPONENTS WinExtras)
    list(APPEND QT_LIBRARIES Qt6::WinExtras)
else()
    find_package(Qt6 REQUIRED COMPONENTS DBus)
    list(APPEND QT_LIBRARIES Qt6::DBus)
    get_target_property(QT_DBUSXML2CPP_EXECUTABLE Qt6::qdbusxml2cpp LOCATION)
endif()

set(QT_QML_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/qml)

add_definitions(${QT_DEFINITIONS})

qt_standard_project_setup()

add_subdirectory(desktop)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

include(ExternalProject)
