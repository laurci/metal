
#include "PID.h"
#include "acc_acq.h"

#include <SparkFun_TB6612.h>

double tilt = 0;

float filtered_output, est_spd;

float sp = 5;



const float alpha2 = 0.95;



#define AIN1 2
#define AIN2 3
#define BIN1 8
#define BIN2 9
#define PWMA 6
#define PWMB 7
#define STBY 10


const int offsetA = 1;
const int offsetB = 1;


Motor motor1 = Motor(AIN1, AIN2, PWMA, offsetA, STBY);
Motor motor2 = Motor(BIN1, BIN2, PWMB, offsetB, STBY);


utils::control::PID pid1, pid2;



void setup() {
  pid1.kp = 20.0;
  pid1.ki = 80.0;
  pid1.kd = 0.0;
  pid1.limit = 55.0;
  pid1.o_limit = 255.0;


  pid2.kp = 0.01;
  pid2.ki = 0.0;
  pid2.kd = 0.0;
  pid2.limit = 2.0;
  pid2.o_limit = 30.0;


  Serial.begin(115200);

  init_acc_acq();
}


void loop() {

  get_angle(tilt);
  double op = 0;
  if (tilt < 30 && tilt > -30) {

    op = pid1.compute(tilt + sp);

    filtered_output = filtered_output * alpha2 + op * (1.0 - alpha2);

    sp = pid2.compute(filtered_output);


    Serial.print("  ");
    Serial.print(pid2.compute(filtered_output));
    Serial.print("  ");


  } else {
    Serial.print("bypass ");
  }


  Serial.println(tilt);

  motor1.drive(op);
  motor2.drive(op);
}