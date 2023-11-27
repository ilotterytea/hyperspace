install(
    TARGETS hyperspace_exe
    RUNTIME COMPONENT hyperspace_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
