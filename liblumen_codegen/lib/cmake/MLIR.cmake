if (LLVM_SOURCE_DIR)
  set(MLIR_SOURCE_DIR ${LLVM_SOURCE_DIR}/projects/mlir)
  set(MLIR_MAIN_SRC_DIR ${MLIR_SOURCE_DIR})
  set(MLIR_BINARY_DIR ${LLVM_BINARY_DIR})
  set(MLIR_INCLUDE_DIR ${LLVM_SOURCE_DIR}/include)
else()
  set(MLIR_BINARY_DIR ${LLVM_BINARY_DIR})
  set(MLIR_INCLUDE_DIR ${MLIR_BINARY_DIR}/include)
endif()
set(MLIR_TABLEGEN_EXE mlir-tblgen)

function(mlir_tablegen ofn)
  tablegen(MLIR ${ARGV} "-I${MLIR_INCLUDE_DIR}")
  set(TABLEGEN_OUTPUT ${TABLEGEN_OUTPUT} ${CMAKE_CURRENT_BINARY_DIR}/${ofn}
      PARENT_SCOPE)
endfunction()

function(whole_archive_link target)
  if("${CMAKE_SYSTEM_NAME}" STREQUAL "Darwin")
    set(link_flags "-L${CMAKE_BINARY_DIR}/lib ")
    FOREACH(LIB ${ARGN})
      string(CONCAT link_flags ${link_flags} "-Wl,-force_load ${CMAKE_BINARY_DIR}/lib/lib${LIB}.a ")
    ENDFOREACH(LIB)
  elseif(MSVC)
    FOREACH(LIB ${ARGN})
      string(CONCAT link_flags ${link_flags} "/WHOLEARCHIVE:${LIB} ")
    ENDFOREACH(LIB)
  else()
    set(link_flags "-L${CMAKE_BINARY_DIR}/lib -Wl,--whole-archive,")
    FOREACH(LIB ${ARGN})
      string(CONCAT link_flags ${link_flags} "-l${LIB},")
    ENDFOREACH(LIB)
    string(CONCAT link_flags ${link_flags} "--no-whole-archive")
  endif()
  set_target_properties(${target} PROPERTIES LINK_FLAGS ${link_flags})
endfunction(whole_archive_link)