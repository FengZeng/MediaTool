cmake_minimum_required(VERSION 3.10)
project(MediaInfoWrap)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -DUNICODE")
set(CMAKE_SYSTEM_PREFIX_PATH "")
set(MAKE_PREFIX_PATH "")

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/MediaInfoLib/Project/CMake ${CMAKE_CURRENT_BINARY_DIR}/MediaInfoLib)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/MediaInfoLib/Source)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/ZenLib/Source)

add_library(mediainfo_wrap STATIC MediaInfoWrap.cpp)
target_link_libraries(mediainfo_wrap mediainfo zen)
set_target_properties(mediainfo_wrap PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}")

install(TARGETS mediainfo_wrap DESTINATION lib)