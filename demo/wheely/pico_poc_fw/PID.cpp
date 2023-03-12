#include "PID.h"
namespace utils {
namespace control {
PID::PID() {
  l_err = 0;
  l_time = 0;
  limit = 0;
  o_limit = 0;
  kp = 0;
  ki = 0;
  kd = 0;
}

float PID::compute(float err) {
  float p, d, dt = (time_us_64() - l_time) / 1000000.0;

  l_time = time_us_64();

  p = err * kp;
  i = i + err * dt * ki;
  d = ((err - l_err) / dt) * kd;

  l_err = err;

  if (i > limit)
    i = limit;
  if (i < -limit)
    i = -limit;

  float
    out = p + i + d;

  if (out > o_limit)
    out = o_limit;
  if (out < -o_limit)
    out = -o_limit;

  return out;
}
}  // namespace control
}  // namespace utils