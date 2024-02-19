cmake_minimum_required(VERSION 3.20)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(IN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/variables.h.in")
set(GENERATED_FILE_PATH "${CMAKE_BINARY_DIR}/GeneratedFiles")
set(OUT_FILE "${GENERATED_FILE_PATH}/variables.h")
configure_file("${IN_FILE}" "${OUT_FILE}" @ONLY)

include_directories(${GENERATED_FILE_PATH})

set(
  SOURCES
  api.cc
  api.h
  main.cc
  ui.cc
  ui.h
  maintainer.cc
  maintainer.h
  model/connection_list_model.cc
  model/connection_list_model.h
  model/connection_model.cc
  model/connection_model.h
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
