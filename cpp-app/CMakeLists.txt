cmake_minimum_required(VERSION 3.16)
project(RustCppApp CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -------- Debug build option --------
option(ENABLE_DEBUG_OUTPUT "Enable debug output" OFF)
if(ENABLE_DEBUG_OUTPUT OR CMAKE_BUILD_TYPE STREQUAL "Debug")
  add_compile_definitions(_DEBUG)
  message(STATUS "Debug output enabled")
endif()

# -------- Options (cache) --------
set(RUST_CRATE_DIR "${CMAKE_SOURCE_DIR}/../rustlib" CACHE PATH "Path to the Rust crate dir")
set(CBINDGEN_CFG   "${CMAKE_SOURCE_DIR}/../cbindgen.toml" CACHE FILEPATH "cbindgen config file")

# -------- Tools --------
find_program(CARGO     cargo     REQUIRED)
find_program(CBINDGEN  cbindgen  REQUIRED)

# ===========================================
# ★ Standardize output directories (relative to root)
#   bin/: executables, lib/: static libraries (.a), include/: headers, obj/: object files (.o)
# ===========================================
set(OUTPUT_ROOT "${CMAKE_SOURCE_DIR}/../output")
set(OUTPUT_BIN  "${OUTPUT_ROOT}/bin")
set(OUTPUT_LIB  "${OUTPUT_ROOT}/lib")
set(OUTPUT_INC  "${OUTPUT_ROOT}/include")
set(OUTPUT_OBJ  "${OUTPUT_ROOT}/obj")

# Specify standard output locations for CMake (default for all targets)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${OUTPUT_BIN}")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${OUTPUT_LIB}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${OUTPUT_LIB}")

# -------- C++ sources (auto-discover) --------
file(GLOB_RECURSE CPP_SOURCES CONFIGURE_DEPENDS
   "${CMAKE_SOURCE_DIR}/src/*.cpp")

if(NOT CPP_SOURCES)
  message(FATAL_ERROR "No C++ sources found in ${CMAKE_SOURCE_DIR}/src")
endif()

# ===========================================
# Rust crate -> staticlib + header
# ===========================================
function(add_rust_staticlib TARGET_NAME CRATE_DIR)
  set(options)
  set(oneValueArgs CBINDGEN_CFG)
  set(multiValueArgs)
  cmake_parse_arguments(RS "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

  if(NOT EXISTS "${CRATE_DIR}/Cargo.toml")
  message(FATAL_ERROR "Cargo.toml not found in ${CRATE_DIR}")
  endif()

  file(GLOB_RECURSE RUST_SOURCES CONFIGURE_DEPENDS
     "${CRATE_DIR}/src/*.rs" "${CRATE_DIR}/build.rs" )

  # Determine build profile and output path
  if(ENABLE_DEBUG_OUTPUT OR CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_PROFILE "debug")
  else()
    set(RUST_PROFILE "release")
  endif()

  # Determine output path according to CARGO_BUILD_TARGET
  if(DEFINED ENV{CARGO_BUILD_TARGET} AND NOT "$ENV{CARGO_BUILD_TARGET}" STREQUAL "")
  set(RUST_TRIPLE "$ENV{CARGO_BUILD_TARGET}")
  set(RUST_LIB "${CRATE_DIR}/target/${RUST_TRIPLE}/${RUST_PROFILE}/lib${TARGET_NAME}.a")
  else()
  set(RUST_LIB "${CRATE_DIR}/target/${RUST_PROFILE}/lib${TARGET_NAME}.a")
  endif()

  # cbindgen header (generated directly to output/include)
  set(GEN_HEADER  "${OUTPUT_INC}/${TARGET_NAME}")

  # Determine Rust build mode based on CMake build type
  if(ENABLE_DEBUG_OUTPUT OR CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_BUILD_FLAGS "")
    set(RUST_BUILD_COMMENT "Building Rust crate (debug): ${TARGET_NAME}")
  else()
    set(RUST_BUILD_FLAGS "--release")
    set(RUST_BUILD_COMMENT "Building Rust crate (release): ${TARGET_NAME}")
  endif()

  add_custom_command(
  OUTPUT "${RUST_LIB}"
  COMMAND ${CMAKE_COMMAND} -E env CARGO_BUILD_TARGET=$ENV{CARGO_BUILD_TARGET}
      "${CARGO}" build ${RUST_BUILD_FLAGS} --manifest-path "${CRATE_DIR}/Cargo.toml"
  WORKING_DIRECTORY "${CRATE_DIR}"
  DEPENDS ${RUST_SOURCES} "${CRATE_DIR}/Cargo.toml"
  BYPRODUCTS "${RUST_LIB}"
  COMMENT "${RUST_BUILD_COMMENT}"
  VERBATIM
  )

  if(RS_CBINDGEN_CFG)
  set(CBINDGEN_CFG_FILE "${RS_CBINDGEN_CFG}")
  else()
  set(CBINDGEN_CFG_FILE "${CBINDGEN_CFG}")
  endif()

  add_custom_command(
  OUTPUT "${GEN_HEADER}"
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${OUTPUT_INC}"
  COMMAND "${CBINDGEN}" "${CRATE_DIR}" -c "${CBINDGEN_CFG_FILE}" -o "${GEN_HEADER}"
  WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
  DEPENDS ${RUST_SOURCES} "${CRATE_DIR}/Cargo.toml" "${CBINDGEN_CFG_FILE}"
  COMMENT "Generating C header via cbindgen: ${GEN_HEADER}"
  VERBATIM
  )

  add_custom_target(${TARGET_NAME}_rust ALL
  DEPENDS "${RUST_LIB}" "${GEN_HEADER}"
  )

  # Expose as IMPORTED static library
  add_library(${TARGET_NAME}_imported STATIC IMPORTED GLOBAL)
  set_target_properties(${TARGET_NAME}_imported PROPERTIES
  IMPORTED_LOCATION "${RUST_LIB}"
  )
  add_dependencies(${TARGET_NAME}_imported ${TARGET_NAME}_rust)

  # After build, copy Rust .a to output/lib
  add_custom_command(TARGET ${TARGET_NAME}_rust POST_BUILD
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${OUTPUT_LIB}"
  COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${RUST_LIB}" "${OUTPUT_LIB}/"
  COMMENT "Export rust staticlib -> ${OUTPUT_LIB}"
  )

  # Return to calling scope
  set(${TARGET_NAME}_HEADER "${GEN_HEADER}" PARENT_SCOPE)
  set(${TARGET_NAME}_LIB     "${RUST_LIB}"  PARENT_SCOPE)
endfunction()

# -------- Add Rust crate once --------
add_rust_staticlib(llm_rust "${RUST_CRATE_DIR}" CBINDGEN_CFG "${CBINDGEN_CFG}")

# ===========================================
# C++: Manage individual .o, static .a, and executable
#   - OBJECT library: cpp_objs (produces each .o)
#   - STATIC library: cpp_static (for external linking, .a)
#   - EXECUTABLE:     cpp_app (final executable)
# ===========================================
add_library(cpp_objs OBJECT ${CPP_SOURCES})
add_dependencies(cpp_objs llm_rust_rust)
target_include_directories(cpp_objs PRIVATE 
  "${CMAKE_SOURCE_DIR}/include"
  "${OUTPUT_INC}"
  "${CMAKE_BINARY_DIR}/third_party/crow/include"
)
# Script to copy individual object files (.o)
set(COPY_OBJS_SCRIPT "${CMAKE_BINARY_DIR}/copy_objs.cmake")
file(WRITE "${COPY_OBJS_SCRIPT}" "
  file(MAKE_DIRECTORY \"${OUTPUT_OBJ}\")
  foreach(obj IN LISTS OBJS)
  get_filename_component(name \"\${obj}\" NAME)
  file(COPY \"\${obj}\" DESTINATION \"${OUTPUT_OBJ}\")
  endforeach()
")

add_custom_target(export_o ALL
  COMMAND ${CMAKE_COMMAND} -DOBJS="$<TARGET_OBJECTS:cpp_objs>" -P "${COPY_OBJS_SCRIPT}"
  DEPENDS cpp_objs
  COMMENT "Export C++ object files -> ${OUTPUT_OBJ}"
)

# Also create C++ static library (.a) for external linking
add_library(cpp_static STATIC $<TARGET_OBJECTS:cpp_objs>)
set_target_properties(cpp_static PROPERTIES OUTPUT_NAME "llmrcpp_app")

# Final executable
add_executable(cpp_app $<TARGET_OBJECTS:cpp_objs>)
add_dependencies(cpp_app llm_rust_rust export_o)
target_link_libraries(cpp_app PRIVATE llm_rust_imported cpp_static)
target_include_directories(cpp_app PRIVATE 
  "${OUTPUT_INC}"
  "${CMAKE_BINARY_DIR}/third_party/crow/include"
)

# macOS framework linking
if(APPLE)
  target_link_libraries(cpp_app PRIVATE "-framework CoreFoundation")
endif()

# Executable name
set_target_properties(cpp_app PROPERTIES OUTPUT_NAME "llmrcpp_app")               # Executable name: llmrcpp_app

# -------- Clean helpers --------
add_custom_target(clean-all
  COMMAND ${CMAKE_COMMAND} -E echo "Cleaning CMake build dir and Cargo target"
  COMMAND ${CMAKE_COMMAND} -E rm -rf "${CMAKE_BINARY_DIR}/*"
  COMMAND ${CARGO} clean --manifest-path "${RUST_CRATE_DIR}/Cargo.toml"
  WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
)