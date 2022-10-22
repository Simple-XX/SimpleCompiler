/*@ requires n >= 0;
  @ ensures \true;
  */
int verify(int n) {
    int x = n;
    int y = 0;
    /*@ loop invariant x >= 0;
      @ loop invariant x + y == n;
      */
    while (x > 0) {
        x = x - 1;
        y = y + 1;
    }
    //@ assert y == n;
    return y;
}