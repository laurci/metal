cmake_minimum_required(VERSION 3.13)

# initialize the SDK based on PICO_SDK_PATH
# note: this must happen before project()

include(pico_sdk_import.cmake)

project(Application)

pico_sdk_init()

add_executable(application
    app/src/main.cpp
)

set(target_libs 
"pico_stdlib"
"pico_multicore"
"hardware_uart"
)

target_link_libraries(application PUBLIC
	${target_libs}
)


# enable usb output, disable uart output
pico_enable_stdio_usb(application 1)
pico_enable_stdio_uart(application 0)
# create map/bin/hex/uf2 file etc.
pico_add_extra_outputs(application)