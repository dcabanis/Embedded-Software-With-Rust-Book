cmake_minimum_required(VERSION 3.28.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(zephyr_rust_led)

target_sources(app PRIVATE src/main.c)
target_include_directories(app PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

set(RUST_ARCHIVE
    "${CMAKE_CURRENT_SOURCE_DIR}/rust/led_logic/target/thumbv7m-none-eabi/release/libled_logic.a"
)

if(NOT EXISTS "${RUST_ARCHIVE}")
    message(FATAL_ERROR
        "Rust library missing: run the Cargo build command first."
    )
endif()

add_library(rust_led STATIC IMPORTED GLOBAL)
set_target_properties(rust_led PROPERTIES
    IMPORTED_LOCATION "${RUST_ARCHIVE}"
)

target_link_libraries(app PRIVATE rust_led)
