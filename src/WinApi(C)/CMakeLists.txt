cmake_minimum_required(VERSION 3.26)
project(C_Wrapper_Fn C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

add_executable(C_Wrapper_Fn
        harddisk.c
        harddisk.h)
