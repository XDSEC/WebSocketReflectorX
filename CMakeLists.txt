cmake_minimum_required(VERSION 3.20)

set(VERSION_MAJOR 0)
set(VERSION_MINOR 2)
set(VERSION_PATCH 11)

execute_process(
  COMMAND git describe --always --dirty
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    OUTPUT_VARIABLE GIT_VERSION
    OUTPUT_STRIP_TRAILING_WHITESPACE
)

set(VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")
set(FULL_VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}.g${GIT_VERSION}")
message(STATUS "Current build version: ${FULL_VERSION}")

project(
  wsrx
    LANGUAGES C CXX
    VERSION ${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}
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
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

if (UNIX OR LINUX)
set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall --std=c++17 -O3 -fPIC" )
set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -O3 -Wall -fPIC" )
elseif(WIN32)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)
endif()

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(QT_MIN_VERSION 6.6)
find_package(Qt6 ${QT_MIN_VERSION} REQUIRED COMPONENTS Core Quick Gui QuickControls2 Widgets Network Concurrent Svg)
set(QT_LIBRARIES Qt6::Core Qt6::Widgets Qt6::Network Qt6::Quick Qt6::Gui Qt6::QuickControls2 Qt6::Concurrent Qt6::Svg)

set(QT_QML_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/qml)

add_definitions(${QT_DEFINITIONS})

qt_standard_project_setup()

set(QAPPLICATION_CLASS QApplication CACHE STRING "Inheritance class for SingleApplication")

add_subdirectory(desktop)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)
