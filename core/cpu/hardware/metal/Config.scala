package metal

import spinal.core._

object Config {
  def spinal = SpinalConfig(
    targetDirectory = "hardware/gen",
    defaultConfigForClockDomains = ClockDomainConfig(
      resetActiveLevel = HIGH
    ),
    onlyStdLogicVectorAtTopLevelIo = true
  )
}
