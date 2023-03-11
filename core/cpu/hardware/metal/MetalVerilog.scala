package metal

import metal.utils._
import java.lang.IllegalArgumentException

object MetalVerilog extends App {
    // TODO: Provide an "empty" program instead of failing
    var programPath = sys.env.get("METAL_BIN_PATH").getOrElse("")
    if(programPath == "") {
        throw new IllegalArgumentException("METAL_BIN_PATH missing")
    }

    println("Loading program from" + programPath)

    val program = Bin.loadProgram(programPath, 1024)

    Config.spinal.generateVerilog({
        val cpu = new top
        cpu.core.ram.ram.initBigInt(program)
        cpu
    })
}