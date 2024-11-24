NPROCS = $(shell grep -c 'processor' /proc/cpuinfo)
MAKEFLAGS += -j$(NPROCS)
SCADS=$(wildcard src/*.scad)
STLS=$(patsubst src/%.scad,dist/%.stl,${SCADS})

.PHONY: dev
dev:
	@find src -type f | entr -c make preview

.PHONY: all
all: ${STLS}

.PHONY: preview
preview: dist/drone.png
	@magick $< sixel:-

dist:
	@mkdir -p dist

dist/%.stl: src/%.scad src dist
	@openscad -qo $@ $<

dist/%.png: src/%.scad src dist
	@openscad -qo $@ $<
