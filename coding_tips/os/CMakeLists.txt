aux_source_directory(src UTIL_SRC)

add_library(util ${UTIL_SRC})

target_include_directories(util
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/inc
)


if(${BUILD_TEST})
    add_subdirectory(test)
endif(${BUILD_TEST})