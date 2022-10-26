/*@ requires \true;
  @ ensures \true;
  */
void fun() {
    int x = 0;
    //@ loop invariant x < 1;
    while (x < 1)
      x = x + 1;
    //@ assert x == 1;
}
