add_library(Qaz)

target_sources(Qaz
  PRIVATE
    qaz_kosu_kosu.cpp
  PUBLIC
    FILE_SET HEADERS
    FILES
      qaz_kosu_kosu.h
)

