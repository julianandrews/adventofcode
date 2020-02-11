cmake_minimum_required(VERSION 3.10)
project(AdventOfCode C CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
add_compile_options("$<$<CONFIG:RELEASE>:-W;-Wall;-O3;-pedantic>")
add_compile_options("$<$<CONFIG:DEBUG>:-W;-Wall;-O0;-g;-pedantic>")

file(GLOB executable_sources RELATIVE ${CMAKE_SOURCE_DIR} "src/d*.cc")

foreach(file_path ${executable_sources})
        get_filename_component(target_name ${file_path} NAME_WE)
        add_executable( ${target_name} ${file_path} )
        target_link_libraries( ${target_name} LINK_PUBLIC AdventOfCodeLib)
        add_custom_target(
                run_${target_name}
                cat ${CMAKE_SOURCE_DIR}/../inputs/${target_name}/input.txt |
                        ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target_name}
                DEPENDS ${target_name} )
endforeach()

add_library(AdventOfCodeLib INTERFACE)
target_include_directories (AdventOfCodeLib INTERFACE src/lib/)
