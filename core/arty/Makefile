current_dir := ${CURDIR}
TOP := top
SOURCES := ${current_dir}/../cpu/hardware/gen/top.v

build:
	f4pga -vv build --flow ./flow.json

include ${current_dir}/../../../projects/f4pga-examples/common/common.mk
