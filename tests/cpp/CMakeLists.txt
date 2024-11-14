# CMake minimum version
cmake_minimum_required(VERSION 3.10)

# Project name and version
project(DisassemblerTest VERSION 1.0)

# Specify C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable
add_executable(cpp main.cpp)

# Set platform-specific configurations
if (WIN32)
    # Windows specific configurations (e.g., libraries or defines)
    message(STATUS "Configuring for Windows")
    # You can add additional Windows-specific libraries here if needed
elseif (APPLE)
    # macOS specific configurations
    message(STATUS "Configuring for macOS")
    # You can add additional macOS-specific libraries here if needed
elseif (UNIX AND NOT APPLE)
    # Linux specific configurations
    message(STATUS "Configuring for Linux")
    # You can add additional Linux-specific libraries here if needed
endif()

# Output message for successful configuration
message(STATUS "Configured DisassemblerTest project")
