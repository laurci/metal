clean:
	rm ./hardware/gen/*.v
	rm ./hardware/gen/*.bin
generate:
	METAL_BIN_PATH="${bin_path}" sbt "runMain metal.MetalVerilog"