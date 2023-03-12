#ifndef _PID_H
#define _PID_H

#include <pico/time.h>

namespace utils
{
    namespace control
    {
        class PID
        {
        private:
            float i, l_err;
            uint64_t l_time;

        public:
            float kp, ki, kd, limit;
            float o_limit;

            PID();
            float compute(float err);
        };
    } // namespace control
} // namespace utils
#endif