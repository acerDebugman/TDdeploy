aux_source_directory(src THREAD_IN_TURN_SRC)

#add_library(thread_in_turn STATIC ${THREAD_IN_TURN_SRC})

#target_include_directories(thread_in_turn
#    PRIVATE inc
#    PUBLIC util
#)

add_executable(gc_thread_in_turn src/thread_in_turn.c)
add_executable(test_pthread src/test_pthread.c)

target_include_directories(gc_thread_in_turn
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/inc
    PUBLIC ${GC_SOURCE_DIR}/include/util
)

target_include_directories(test_pthread
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/inc
    PUBLIC ${GC_SOURCE_DIR}/include/util
)

#target_link_libraries(gc_thread_in_turn thread_in_turn util)
target_link_libraries(gc_thread_in_turn util)
target_link_libraries(test_pthread util)
