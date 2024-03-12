cmake_minimum_required(VERSION 3.10)
project(Y VERSION 0.1)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)

file(GLOB SOURCES "src/*.c")

add_executable(Y ${SOURCES})
